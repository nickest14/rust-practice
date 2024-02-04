use super::formats::Format;
use crate::{configuration::Settings, task::Task, utils};

pub fn print_task(task: &Task, format: Option<Format>, settings: &Settings) {
    let tasks = vec![task];
    print_tasks(tasks, format, true, true, settings)
}

pub fn print_tasks(
    tasks: Vec<&Task>,
    format: Option<Format>,
    show_descriptions: bool,
    show_urls: bool,
    settings: &Settings,
) {
    match format {
        Some(Format::Json) => println!(
            "{}",
            serde_json::to_string(&tasks).expect("Falied to serialize tasks to json")
        ),
        Some(Format::JsonPretty) => println!(
            "{}",
            serde_json::to_string_pretty(&tasks).expect("Falied to serialize tasks to json"),
        ),
        _ => {
            let longest_name = tasks.iter().map(|t| t.name.len()).max().unwrap_or(0);
            let longest_date = tasks
                .iter()
                .map(|t| utils::date_to_display_str(&t.date, settings).len())
                .max()
                .unwrap_or(0);
            let longest_repeat = tasks
                .iter()
                .map(|t| t.repeats.to_string().len())
                .max()
                .unwrap_or(0);
            let longest_group = tasks
                .iter()
                .map(|t| t.group.as_deref().unwrap_or_default().len())
                .max()
                .unwrap_or(0);

            // Print header
            print!("{:width$}  ", "Name", width = longest_name + 10);
            print!("{:width$}  ", "Date", width = longest_date);
            print!("{:width$}\t", "Repeats", width = longest_repeat);
            print!("{:width$}\t", "Group", width = longest_group);

            if show_descriptions {
                print!("Description  ")
            }
            if show_urls {
                print!("Url  ")
            }
            println!();

            // Print tasks
            for task in tasks {
                let complete = task.complete;
                let x = settings.icons.get_complete_icon(complete);
                let name = task.name.clone();
                let id = task.id.unwrap();
                let name_id = format!("{} {} ({})", x, name, id);
                let width = longest_name + 10;
                print!("{:width$}  ", name_id, width = width);

                let date = utils::date_to_display_str(&task.date, settings);
                print!("{:width$}  ", date, width = longest_date);

                let repeats = &task.repeats;
                print!("{:width$}\t", repeats, width = longest_repeat);

                let group = &task.group.as_deref().unwrap_or_default();
                print!("{:width$}\t", group, width = longest_group);

                if show_descriptions {
                    let description = task.description.clone();
                    print!("{}  ", description.unwrap_or(String::from("")));
                }

                if show_urls {
                    let url = task.url.clone();
                    print!("{}  ", url.unwrap_or(String::from("")));
                }

                println!();
            }
        }
    }
}
