use clap::Parser;

mod app;
use app::*;

/// Simple ToDo program
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the activity
    #[arg(short, long)]
    name: String,

    /// Priority of the activity
    #[arg(short, long, default_value_t = 1)]
    priority: u8,

    /// Status of activity
    #[arg(short, long, default_value_t = false)]
    completed: bool,
}

fn main() {
    let args = Args::parse();

    let new_setup =
        Todo::new(args.name, args.priority, args.completed).expect("Failed to create file.");

    new_setup.write_to_file().expect("Failed to write to file.");
}
