use crate::task::{CronArgument, Task, TaskExecutor};
use apalis::{
    layers::{retry::RetryPolicy, WorkerBuilderExt},
    prelude::{Data, WorkerBuilder, WorkerFactoryFn},
};
use apalis_cron::CronStream;
use std::collections::HashMap;
use tokio::signal;
use tracing::{error, info};

pub async fn create_worker(task: Task) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let schedule = task
        .get_schedule()
        .map_err(|e| format!("Invalid schedule for task '{}': {e}", task.name))?;

    let worker = WorkerBuilder::new(task.name.clone())
        .retry(RetryPolicy::default())
        .data(task.clone())
        .backend(CronStream::new(schedule))
        .build_fn(perform_task);

    worker.run().await;
    Ok(())
}

pub async fn run_all_tasks(tasks: HashMap<String, Task>) {
    if tasks.is_empty() {
        return;
    }

    let mut handles = Vec::new();

    for (name, task) in tasks {
        let task_handle = tokio::spawn(async move {
            info!("Starting task: {name}");
            if let Err(e) = create_worker(task).await {
                error!("Error running task '{name}': {e}");
            }
        });
        handles.push(task_handle);
    }

    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Received Ctrl+C, shutting down...");
        }
        _ = terminate => {
            info!("Received terminate signal, shutting down...");
        }
    }

    for handle in handles {
        handle.abort();
    }

    info!("All tasks stopped.");
}

pub async fn perform_task(job: CronArgument, data: Data<Task>) {
    data.execute(job).await;
}
