use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Error;

const FILE_PATH: &str = "/home/buzzkill/Documents/Projects/rust/todo/todoFile.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    pub id: usize,
    pub name: String,
    pub priority: u8,
    pub completed: bool,
}

impl Todo {
    pub fn new(id: usize, name: String, priority: u8, completed: bool) -> Result<Self, Error> {
        Self::create_if_not_present().expect("Failed to create file.");

        Ok(Self {
            id,
            name,
            priority,
            completed,
        })
    }

    pub fn read_from_file() -> Result<Vec<Todo>, Error> {
        Self::create_if_not_present()?;

        let file_content = std::fs::read_to_string(FILE_PATH)?;
        if file_content.is_empty() {
            return Ok(Vec::new());
        }

        let content: Vec<Todo> = serde_json::from_str(&file_content).unwrap_or_else(|_| Vec::new());

        Ok(content)
    }

    pub fn write_to_file(&self) -> Result<(), Error> {
        Self::create_if_not_present()?;

        let mut content = Self::read_from_file()?;

        content.push(self.clone());

        let json = serde_json::to_string(&content)?;
        std::fs::write(FILE_PATH, json)?;

        Ok(())
    }

    pub fn remove_activity(id: usize) -> Result<(), Error> {
        Self::create_if_not_present()?;

        let mut activities = Self::read_from_file()?;

        if id == 0 || id > activities.len() {
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid ID: {}, Valid Range: 1-{}", id, activities.len()),
            ));
        }

        let removed_activity = activities.remove(id - 1);

        let json = serde_json::to_string_pretty(&activities)?;
        std::fs::write(FILE_PATH, json)?;

        println!("Removed: {}", removed_activity.name);

        Ok(())
    }

    pub fn edit_activity(
        id: usize,
        name: Option<String>,
        priority: Option<u8>,
        completed: Option<bool>,
    ) -> Result<(), Error> {
        Self::create_if_not_present()?;

        let mut activities = Self::read_from_file()?;

        if id == 0 || id > activities.len() {
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid ID: {}, Valid range is: 1-{}", id, activities.len()),
            ));
        }

        let activity = &mut activities[id - 1];

        if let Some(new_name) = name {
            activity.name = new_name;
        }

        if let Some(new_priority) = priority {
            activity.priority = new_priority;
        }

        if let Some(new_completed) = completed {
            activity.completed = new_completed;
        }

        let json = serde_json::to_string_pretty(&activities)?;
        std::fs::write(FILE_PATH, json)?;

        println!("Updated activity. ID: {}", id);
        Ok(())
    }

    pub fn search_activities(
        id: Option<usize>,
        name: Option<String>,
        priority: Option<u8>,
        completed: Option<bool>,
    ) -> Result<(), Error> {
        Self::create_if_not_present()?;

        let all_activities = Self::read_from_file()?;

        if all_activities.is_empty() {
            println!("No activities found.");
            return Ok(());
        }

        let filtered_activities: Vec<&Todo> = all_activities
            .iter()
            .filter(|activity| {
                let id_matches = match id {
                    Some(search) => activity.id == search,
                    None => true,
                };

                let name_matches = match &name {
                    Some(search) => activity
                        .name
                        .to_lowercase()
                        .contains(&search.to_lowercase()),
                    None => true,
                };

                let priority_matches = match priority {
                    Some(search) => activity.priority == search,
                    None => true,
                };

                let completed_matches = match completed {
                    Some(search) => activity.completed == search,
                    None => true,
                };

                id_matches && name_matches && priority_matches && completed_matches
            })
            .collect();

        if filtered_activities.is_empty() {
            println!("No matching activities found.");
            return Ok(());
        }

        println!(
            "{:<4} {:<30} {:<10} {:<10}",
            "ID", "Name", "Priority", "Status"
        );
        println!("{}", "-".repeat(60));

        for activity in filtered_activities.iter() {
            let status = if activity.completed {
                "Done"
            } else {
                "Not Done"
            };

            println!(
                "{:<4} {:<30} {:<10} {:<10}",
                activity.id,
                if activity.name.len() > 27 {
                    format!("{}...", &activity.name[..27])
                } else {
                    activity.name.clone()
                },
                activity.priority,
                status
            );
        }

        println!("Found {} matching activities.", filtered_activities.len());
        Ok(())
    }

    pub fn list_activities(&self) -> Result<(), Error> {
        Self::create_if_not_present()?;

        let content = Self::read_from_file().expect("Error reading from file.");

        if content.is_empty() {
            println!("Empty records.");
            return Ok(());
        }

        println!(
            "{:<4} {:<30} {:<10} {:<10}",
            "ID", "Name", "Priority", "Status"
        );
        println!("{}", "-".repeat(60));

        for (index, todo) in content.iter().enumerate() {
            let status = if todo.completed { "Done" } else { "Not Done" };
            println!(
                "{:<4} {:<30} {:<10} {:<10}",
                index + 1,
                if todo.name.len() > 27 {
                    format!("{}...", &todo.name[..27])
                } else {
                    todo.name.clone()
                },
                todo.priority,
                status
            );
        }

        Ok(())
    }

    fn create_if_not_present() -> Result<(), Error> {
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(FILE_PATH)?;

        Ok(())
    }
}
