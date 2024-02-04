use crate::app::App;

use crate::cli::cli_utils;
use crate::cli::formats::Format;
use crate::task_form::TaskForm;
use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// The name of the new task
    name: String,
    /// The date the task is due
    #[arg(long)]
    date: Option<String>,
    /// How often the task repeats
    #[arg(long)]
    repeats: Option<String>,
    /// The group the task belongs to
    #[arg(long)]
    group: Option<String>,
    /// A description or url for your task
    #[arg(long)]
    description: Option<String>,
    /// A url for your task
    #[arg(long)]
    url: Option<String>,
    /// The format to display the new task with
    #[arg(long)]
    format: Option<Format>,
}

pub fn run(mut app: App, args: Args) -> Result<()> {
    let Args {
        name,
        format,
        date,
        repeats,
        group,
        description,
        url,
    } = args;
    let mut task_form = TaskForm {
        id: None,
        name,
        date: date.unwrap_or("".to_string()),
        repeats: repeats.unwrap_or("".to_string()),
        group: group.unwrap_or("".to_string()),
        description: description.unwrap_or("".to_string()),
        url: url.unwrap_or("".to_string()),
    };
    let task = task_form.submit(&app.settings)?;
    let id = app.add_task(task);
    let task = app.get_task(id).unwrap();
    cli_utils::print_task(task, format, &app.settings);
    Ok(())
}
