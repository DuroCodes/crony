mod cli;
mod config;
mod errors;
mod interactive;
mod parser;
mod task;
mod worker;

use cli::handle_cli_command;
use config::TaskConfig;
use errors::Result;
use interactive::handle_interactive_mode;

use clap::{Parser, Subcommand};
use cliclack::{intro, select};
use console::style;

#[derive(Parser)]
#[command(name = "crony")]
#[command(about = "A simple cron job manager")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Clone, Debug, Eq, PartialEq)]
pub enum Commands {
    Create,
    List,
    Delete,
    Edit,
    Run,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        return handle_cli_command(command);
    }

    intro(style(" crony ").on_cyan().black())?;

    let mut config = TaskConfig::load()?;
    let mode = select("select a mode")
        .item(Some(Commands::Create), "create a task", "")
        .item(Some(Commands::List), "list all tasks", "")
        .item(Some(Commands::Delete), "delete a task", "")
        .item(Some(Commands::Edit), "edit a task", "")
        .item(Some(Commands::Run), "run all tasks in background", "")
        .interact()?;

    handle_interactive_mode(mode, &mut config)?;
    Ok(())
}
