use clap::{Parser, Subcommand};
use std::process;

#[cfg(feature = "gui")]
mod gui;

#[derive(Parser, Debug)]
#[command(version, long_version = None, about, long_about = None, name = "Surreal Nyvo")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[cfg(feature = "gui")]
    /// Launch the GUI application
    Gui { path: Option<String> },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        #[cfg(feature = "gui")]
        Some(Commands::Gui { .. }) => {
            gui::launch();
        }
        None => {
            println!("No command provided. Use --help for more information.");

            #[cfg(feature = "gui")]
            println!("\nTo launch the GUI, use the `gui` subcommand instead:\nnyvo gui");
            process::exit(1);
        }
    }
}
