use std::collections::HashMap;

use crate::{
    configuration::{get_db_file, Settings},
    task::Task,
    utils,
};

pub type Id = usize;

pub struct App {
    pub tasks: HashMap<Id, Task>,
    pub settings: Settings,
    pub current_id: usize,
}

impl App {
    pub fn new(settings: Settings) -> Self {
        let tasks: HashMap<Id, Task> = utils::load_tasks(get_db_file());
        let current_id = tasks.iter().map(|(&k, _)| k).max().unwrap_or(0);
        App {
            tasks,
            settings,
            current_id,
        }
    }
}
