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

    pub fn add_task(&mut self, mut t: Task) -> Id {
        let new_id = match t.id {
            Some(id) => {
                self.tasks.insert(id, t);
                // self.save_state();
                id
            }
            None => {
                let new_id = self.get_next_id();
                t.id = Some(new_id);
                self.tasks.insert(new_id, t);
                new_id
            }
        };
        self.save_state();
        new_id
    }

    pub fn get_task(&self, id: Id) -> Option<&Task> {
        self.tasks.get(&id)
    }

    fn save_state(&mut self) {
        utils::save_tasks(get_db_file(), self);
    }

    fn get_next_id(&mut self) -> usize {
        self.current_id += 1;
        self.current_id
    }
}
