use crate::repeat::Repeat;
use chrono::{Date, DateTime, Local, TimeZone};
use serde::{Deserialize, Serialize};

pub fn serialize_dt<S>(date: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let s = date.format("%+").to_string();
    serializer.serialize_str(&s)
}

pub fn deserialize_dt<'de, D>(deserializer: D) -> Result<DateTime<Local>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let dt = Local.datetime_from_str(&s, "%+").unwrap();
    Ok(dt)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: Option<usize>,
    pub name: String,
    #[serde(serialize_with = "serialize_dt", deserialize_with = "deserialize_dt")]
    pub date: DateTime<Local>,
    pub repeats: Repeat,
    pub group: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub complete: bool,
}

impl Task {
    pub fn set_id(&mut self, id: Option<usize>) {
        self.id = id;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_date(&mut self, date: DateTime<Local>) {
        self.date = date;
    }

    pub fn set_repeats(&mut self, repeats: Repeat) {
        self.repeats = repeats;
    }

    pub fn set_group(&mut self, group: String) {
        self.group = Some(group);
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    pub fn set_url(&mut self, url: String) {
        self.url = Some(url);
    }
}

impl Default for Task {
    fn default() -> Self {
        Self {
            id: None,
            name: "".to_string(),
            date: Local::now(),
            repeats: Repeat::Never,
            group: None,
            description: None,
            url: None,
            complete: false,
        }
    }
}
