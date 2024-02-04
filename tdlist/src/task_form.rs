use crate::configuration::Settings;
use crate::repeat::Repeat;
use crate::task::Task;
use crate::utils;
use anyhow::{anyhow, Context, Result};

#[derive(Default)]
pub struct TaskForm {
    pub id: Option<usize>,
    pub name: String,
    pub date: String,
    pub repeats: String,
    pub group: String,
    pub description: String,
    pub url: String,
}

impl TaskForm {
    pub fn submit(&mut self, settings: &Settings) -> Result<Task> {
        if self.name.is_empty() {
            return Err(anyhow!("Task name cannot be empty"));
        }
        let mut task = Task::default();
        let repeat = Repeat::parse_from_str(&self.repeats).context("Invalid repeat format")?;
        let date = utils::parse_date(&self.date, settings).unwrap_or(utils::get_today());
        // let a: usize = 3;
        // task.set_id(Some(a));
        task.set_id(self.id);
        task.set_name(self.name.clone());
        task.set_date(date);
        task.set_repeats(repeat);
        if !self.group.is_empty() {
            task.set_group(self.group.clone());
        }
        if !self.description.is_empty() {
            task.set_description(self.description.clone());
        }
        if !self.url.is_empty() {
            task.set_url(self.url.clone());
        }
        Ok(task)
    }
}
