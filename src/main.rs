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

    /// List all ToDo items
    List,

    /// Remove an activity
    Remove {
        #[arg(short, long)]
        id: usize,
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

        Some(Commands::List) => {
            let todo =
                Todo::new(0, String::new(), 0, false).expect("Failed to create temporary ToDo.");

            todo.list_activities().expect("Failed to read from file.");
        }

        Some(Commands::Remove { id }) => {
            Todo::remove_activity(*id).expect("Failed to remove activity.");
        }

        None => {
            eprintln!("No command specified. Use --help for usage instructions.");
        }
    }
}
