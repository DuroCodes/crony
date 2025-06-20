use crate::errors::{CronyError, Result};
use crate::task::Task;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf};

#[derive(Serialize, Deserialize, Default)]
pub struct TaskConfig {
    pub tasks: HashMap<String, Task>,
}

impl TaskConfig {
    pub fn load() -> Result<Self> {
        let config_path = get_config_path();

        if !config_path.exists() {
            if let Some(parent) = config_path.parent() {
                fs::create_dir_all(parent)?;
            }
            return Ok(TaskConfig::default());
        }

        let content = fs::read_to_string(config_path)?;
        let config: TaskConfig = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let config_path = get_config_path();

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        fs::write(config_path, content)?;
        Ok(())
    }

    pub fn add_task(&mut self, name: String, task: Task) -> Result<()> {
        if self.tasks.contains_key(&name) {
            return Err(CronyError::Task(format!("Task '{name}' already exists")));
        }
        self.tasks.insert(name, task);
        self.save()
    }

    pub fn remove_task(&mut self, name: &str) -> Result<Option<Task>> {
        let task = self.tasks.remove(name);
        self.save()?;
        Ok(task)
    }

    pub fn update_task(&mut self, name: &str, task: Task) -> Result<()> {
        if !self.tasks.contains_key(name) {
            return Err(CronyError::Task(format!("Task '{name}' does not exist")));
        }
        self.tasks.insert(name.to_string(), task);
        self.save()
    }

    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    pub fn len(&self) -> usize {
        self.tasks.len()
    }
}

fn get_config_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home)
        .join(".config")
        .join("crony")
        .join("tasks.toml")
}
