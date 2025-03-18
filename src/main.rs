use clap::{Parser, Subcommand};

mod app;
use app::*;

/// ToDo
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Add a new activity item
    Add {
        /// ID of the activity
        #[arg(short, long)]
        id: usize,

        /// Name of the activity
        #[arg(short, long)]
        name: String,

        /// Priority of the activity
        #[arg(short, long, default_value_t = 1)]
        priority: u8,

        /// Status of activity
        #[arg(short, long, default_value_t = false)]
        completed: bool,
    },

    /// List all activities
    List {
        /// Sort by a specific field
        #[arg(short, long, value_enum, default_value_t = SortBy::ID)]
        sort: SortBy,

        /// Reverse the sort order
        #[arg(short, long, default_value_t = false)]
        reverse: bool,
    },

    /// Remove an activity
    Remove {
        #[arg(short, long)]
        id: usize,
    },

    /// Edit an activity
    Edit {
        /// ID of the activity
        #[arg(short, long)]
        id: usize,

        /// Name of the activity (optional)
        #[arg(short, long)]
        name: Option<String>,

        /// Priority of the activity (optional)
        #[arg(short, long)]
        priority: Option<u8>,

        /// Status of the activity (optional)
        #[arg(short, long)]
        completed: Option<bool>,
    },

    /// Search for an activity
    Search {
        /// Search by ID (optional)
        #[arg(short, long)]
        id: Option<usize>,

        /// Search by name (optional)
        #[arg(short, long)]
        name: Option<String>,

        /// Search by priority (optional)
        #[arg(short, long)]
        priority: Option<u8>,

        /// Search by completion status (optional)
        #[arg(short, long)]
        completed: Option<bool>,
    },
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Some(Commands::Add {
            id,
            name,
            priority,
            completed,
        }) => {
            let new_todo = Todo::new(*id, name.clone(), *priority, *completed)
                .expect("Failed to create ToDo item.");

            new_todo.write_to_file().expect("Failed to wrtie to file.");

            println!("Added: {}", name);
        }

        Some(Commands::List { sort, reverse }) => {
            Todo::list_activities(sort.clone(), *reverse).expect("Failed to list activities.");
        }

        Some(Commands::Remove { id }) => {
            Todo::remove_activity(*id).expect("Failed to remove activity.");
        }

        Some(Commands::Edit {
            id,
            name,
            priority,
            completed,
        }) => {
            Todo::edit_activity(*id, name.clone(), *priority, *completed)
                .expect("Failed to edit activity");
        }

        Some(Commands::Search {
            id,
            name,
            priority,
            completed,
        }) => {
            let id_value = *id;
            let name_clone = name.clone();
            let priority_value = priority.map(|p| p);
            let completed_value = completed.map(|c| c);

            Todo::search_activities(id_value, name_clone, priority_value, completed_value)
                .expect("Failed to search activity.");
        }

        None => {
            eprintln!("No command specified. Use --help for usage instructions.");
        }
    }
}
