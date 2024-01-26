use crate::app::App;
use anyhow::Result;
use clap::Parser;

mod add;
mod complete;
mod config;
mod delete;
mod ls;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Parser)]
enum Command {
    /// Lists all the tasks
    Ls(ls::Args),
    /// Adds a task to your todos
    Add(add::Args),
    /// Deletes a task from your todos
    Delete(delete::Args),
    /// Marks a task as complete or incomplete
    Complete(complete::Args),
    /// Sets default configurations
    Config(config::Args),
}

pub fn start_cli(app: App) -> Result<()> {
    let args = Args::parse();
    match args.command {
        Command::Ls(args) => ls::run(app, args),
        Command::Add(args) => add::run(app, args),
        Command::Delete(args) => delete::run(app, args),
        Command::Complete(args) => complete::run(app, args),
        Command::Config(args) => config::run(app, args),
    }
}
