use thiserror::Error;

#[derive(Error, Debug)]
pub enum CronyError {
    #[error("Task error: {0}")]
    Task(String),

    #[error("Schedule parsing error: {0}")]
    Schedule(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TOML parsing error: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("TOML serialization error: {0}")]
    TomlSer(#[from] toml::ser::Error),

    #[error("CLI interaction error: {0}")]
    Cli(String),
}

pub type Result<T> = std::result::Result<T, CronyError>;
