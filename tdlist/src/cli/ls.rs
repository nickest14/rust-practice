use std::collections::HashMap;

use crate::app::{App, Id};
use crate::task::Task;

use anyhow::Result;
use clap::{Parser, ValueEnum};

use super::cli_utils;
use super::formats::Format;

#[derive(Parser)]
pub struct Args {
    /// The format to print tasks with
    #[arg(short, long)]
    format: Option<Format>,
    /// Whether to show complete tasks
    #[arg(short, long)]
    show_complete: bool,
    /// Whether to show task descriptions
    #[arg(long)]
    show_descriptions: bool,
    /// Whether to show task urls
    #[arg(long)]
    show_urls: bool,
    /// Filter tasks by relative date
    #[arg(long)]
    date_filter: Option<DateFilter>,
    /// Filter tasks by date
    #[arg(long)]
    date: Option<String>,
    /// Filter by group
    #[arg(long)]
    group: Option<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum DateFilter {
    All,
    Today,
    Past,
    TodayAndPast,
    Next24,
}

pub fn run(app: App, args: Args) -> Result<()> {
    // TODO:
    let Args {
        format,
        show_complete,
        show_descriptions,
        show_urls,
        date_filter,
        date,
        group,
    } = args;

    let tasks: HashMap<Id, Task> = if !show_complete {
        app.tasks.into_iter().filter(|(_, t)| !t.complete).collect()
    } else {
        app.tasks
    };

    // TODO: Add filter function

    let mut tasks_vec = tasks.values().collect::<Vec<_>>();
    tasks_vec.sort_by(|a, b| a.date.cmp(&b.date).then_with(|| a.name.cmp(&b.name)));

    cli_utils::print_tasks(
        tasks_vec,
        format,
        show_descriptions,
        show_urls,
        &app.settings,
    );
    Ok(())
}
