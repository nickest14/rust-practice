// use anyhow::{anyhow, Result};
// use chrono::{DateTime, Local, NaiveDate, TimeZone, Timelike};

use crate::app::Id;
use crate::task::Task;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub fn load_tasks(file: PathBuf) -> HashMap<Id, Task> {
    let file = fs::read_to_string(file).expect("Unable to read file");
    let tasks_map: HashMap<Id, Task> =
        serde_json::from_str(&file).expect("Unable to parse database file");
    tasks_map
}
