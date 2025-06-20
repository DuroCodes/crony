use crate::{
    config::TaskConfig,
    errors::{CronyError, Result},
    task::{parse_schedule, Task},
    Commands,
};
use cliclack::{input, log::info, select};
use console::style;

pub fn handle_interactive_mode(mode: Option<Commands>, config: &mut TaskConfig) -> Result<()> {
    match mode {
        Some(Commands::Create) => handle_create(config),
        Some(Commands::List) => handle_list(config),
        Some(Commands::Delete) => handle_delete(config),
        Some(Commands::Edit) => handle_edit(config),
        Some(Commands::Run) => handle_run(config),
        _ => panic!("invalid mode selected"),
    }
}

fn handle_create(config: &mut TaskConfig) -> Result<()> {
    let existing_task_names: Vec<String> = config.tasks.keys().cloned().collect();

    let name: String = input("enter a name for your task")
        .validate(move |input: &String| match input.trim() {
            "" => Err("task name cannot be empty".into()),
            name if existing_task_names.contains(&name.to_string()) => {
                Err(format!("task name '{name}' already exists."))
            }
            _ => Ok(()),
        })
        .interact()?;

    let raw_cron: String = input("input a cron expression for the task")
        .placeholder("0 * * * * *")
        .validate(|input: &String| {
            parse_schedule(input)
                .map(|_| ())
                .map_err(|_| "invalid cron expression")
        })
        .interact()?;

    let command: String = input("input a command to run")
        .placeholder("echo 'hello world'")
        .validate(|input: &String| match input.trim() {
            "" => Err("command cannot be empty"),
            _ => Ok(()),
        })
        .interact()?;

    let task = Task::new(name.clone(), raw_cron, command);
    config.add_task(name, task.clone())?;

    info(format!(
        "task '{}' created with schedule '{}' and command '{}'",
        style(&task.name).bold().green(),
        style(&task.schedule).bold().yellow(),
        style(&task.command).bold().blue()
    ))?;

    Ok(())
}

fn handle_list(config: &TaskConfig) -> Result<()> {
    if config.is_empty() {
        info("no tasks configured")?;
    } else {
        config
            .tasks
            .iter()
            .enumerate()
            .for_each(|(i, (name, task))| {
                let _ = info(format!(
                    "{}. {} | {} | {}",
                    style(i + 1).bold().cyan(),
                    style(name).bold().green(),
                    style(&task.schedule).bold().yellow(),
                    style(&task.command).bold().blue()
                ));
            });
    }
    Ok(())
}

fn handle_delete(config: &mut TaskConfig) -> Result<()> {
    if config.is_empty() {
        info("no tasks to delete")?;
        return Ok(());
    }

    let task_names: Vec<_> = config.tasks.keys().cloned().collect();
    let task_name_refs: Vec<_> = task_names.iter().map(|s| s.as_str()).collect();

    let selected_task = select("select a task to delete")
        .items(
            &task_name_refs
                .iter()
                .map(|name| (Some(*name), *name, ""))
                .collect::<Vec<_>>(),
        )
        .interact()?;

    if let Some(task_name) = selected_task {
        config.remove_task(task_name)?;
        info(format!(
            "task '{}' deleted successfully!",
            style(task_name).bold().red()
        ))?;
    }

    Ok(())
}

fn handle_edit(config: &mut TaskConfig) -> Result<()> {
    if config.is_empty() {
        info("no tasks to edit")?;
        return Ok(());
    }

    let task_names: Vec<String> = config.tasks.keys().cloned().collect();
    let task_name_refs: Vec<&str> = task_names.iter().map(|s| s.as_str()).collect();

    let selected_task = select("select a task to edit")
        .items(
            &task_name_refs
                .iter()
                .map(|name| (Some(*name), *name, ""))
                .collect::<Vec<_>>(),
        )
        .interact()?;

    if let Some(task_name) = selected_task {
        let (current_schedule, current_command) = {
            let task = config.tasks.get(task_name).unwrap();
            (task.schedule.clone(), task.command.clone())
        };

        let new_schedule: String = input("input a new cron expression for the task")
            .default_input(&current_schedule)
            .validate(|input: &String| {
                parse_schedule(input)
                    .map(|_| ())
                    .map_err(|_| "invalid cron expression")
            })
            .interact()?;

        let new_command: String = input("input a new command to run")
            .default_input(&current_command)
            .validate(|input: &String| match input.trim() {
                "" => Err("command cannot be empty"),
                _ => Ok(()),
            })
            .interact()?;

        let updated_task = Task::new(task_name.into(), new_schedule, new_command);
        config.update_task(task_name, updated_task.clone())?;

        info(format!(
            "task '{}' updated with schedule '{}' and command '{}'",
            style(&updated_task.name).bold().green(),
            style(&updated_task.schedule).bold().yellow(),
            style(&updated_task.command).bold().blue()
        ))?;
    }

    Ok(())
}

fn handle_run(config: &TaskConfig) -> Result<()> {
    if config.is_empty() {
        info("no tasks to run")?;
        return Ok(());
    }

    info(format!(
        "starting {} task(s) in background...",
        style(config.len()).bold().cyan()
    ))?;

    tracing_subscriber::fmt::init();

    let rt = tokio::runtime::Runtime::new()
        .map_err(|e| CronyError::Task(format!("Failed to create runtime: {}", e)))?;

    let tasks = config.tasks.clone();
    rt.block_on(async {
        crate::worker::run_all_tasks(tasks).await;
    });

    Ok(())
}
