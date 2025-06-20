use crate::{config::TaskConfig, errors::{CronyError, Result}, worker, Commands};
use tracing::info;

pub fn handle_cli_command(command: Commands) -> Result<()> {
    let config = TaskConfig::load()?;

    match command {
        Commands::Run => handle_run_command(config),
        Commands::List => handle_list_command(config),
        Commands::Create => Err(CronyError::Cli(
            "Interactive create mode not supported via CLI. Use 'crony' without arguments.".into(),
        )),
        Commands::Delete => Err(CronyError::Cli(
            "Interactive delete mode not supported via CLI. Use 'crony' without arguments.".into(),
        )),
        Commands::Edit => Err(CronyError::Cli(
            "Interactive edit mode not supported via CLI. Use 'crony' without arguments.".into(),
        )),
    }
}

fn handle_run_command(config: TaskConfig) -> Result<()> {
    tracing_subscriber::fmt::init();

    if config.is_empty() {
        info!("No tasks to run");
        return Ok(());
    }

    info!("Starting {} task(s) in background...", config.len());

    let rt = tokio::runtime::Runtime::new()
        .map_err(|e| CronyError::Task(format!("Failed to create runtime: {}", e)))?;

    rt.block_on(async {
        worker::run_all_tasks(config.tasks).await;
    });

    Ok(())
}

fn handle_list_command(config: TaskConfig) -> Result<()> {
    if config.is_empty() {
        println!("no tasks configured");
    } else {
        println!("configured tasks:");
        for (i, (name, task)) in config.tasks.iter().enumerate() {
            println!("{}. {name} | {} | {}", i + 1, task.schedule, task.command);
        }
    }

    Ok(())
}
