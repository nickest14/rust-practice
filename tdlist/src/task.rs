use crate::repeat::Repeat;
use chrono::{DateTime, Local, TimeZone};
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

impl Task {}

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
