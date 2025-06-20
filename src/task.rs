use crate::errors::{CronyError, Result};
use crate::parser::parse_natural_language;
use apalis_cron::Schedule;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::{future::Future, str::FromStr};
use tracing::{error, info};

#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
    pub name: String,
    pub schedule: String,
    pub command: String,
}

impl Task {
    pub fn new(name: String, schedule: String, command: String) -> Self {
        Self {
            name,
            schedule,
            command,
        }
    }

    pub fn get_schedule(&self) -> Result<Schedule> {
        parse_schedule(&self.schedule)
    }
}

impl TaskExecutor for Task {
    fn execute(&self, _: CronArgument) -> impl Future<Output = ()> + Send {
        let command = self.command.clone();
        let name = self.name.clone();

        async move {
            info!("Executing task '{name}' with command: {command}");
            if let Err(e) = run_command(&command) {
                error!("Failed to execute task '{name}': {e}");
            }
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct CronArgument;

impl From<DateTime<Local>> for CronArgument {
    fn from(_value: DateTime<Local>) -> Self {
        CronArgument
    }
}

pub trait TaskExecutor {
    fn execute(&self, argument: CronArgument) -> impl Future<Output = ()> + Send;
}

pub fn parse_schedule(input: &str) -> Result<Schedule> {
    if let Ok(cron) = parse_natural_language(input) {
        return Schedule::from_str(&cron.to_string())
            .map_err(|e| CronyError::Schedule(e.to_string()));
    }

    Schedule::from_str(input).map_err(|e| CronyError::Schedule(format!("Invalid schedule: {e}")))
}

fn run_command(command: &str) -> Result<std::process::ExitStatus> {
    std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .spawn()
        .map_err(|e| CronyError::Task(format!("Failed to run command '{command}': {e}")))
        .and_then(|mut child| {
            child.wait().map_err(|e| {
                CronyError::Task(format!("Failed to wait for command '{command}': {e}"))
            })
        })
}
