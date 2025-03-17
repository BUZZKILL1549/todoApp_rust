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

    pub fn list_activities(&self) -> Result<(), Error> {
        Self::create_if_not_present()?;

        let content = Self::read_from_file().expect("Error reading from file.");

        if content.is_empty() {
            println!("Empty records.");
            return Ok(());
        }

        println!(
            "{:<4} {:<30} {:<10} {:<10}",
            "ID", "Task", "Priority", "Status"
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
