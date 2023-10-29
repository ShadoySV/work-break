#![windows_subsystem = "windows"]

use clap::{Parser, Subcommand};
use interprocess::local_socket::LocalSocketStream;

mod activities;
mod app;
mod math;

use app::{socket_name, App, Ipc};

/// Work and rest time balancer taking into account your current and today strain
#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run the app in background to send you notifications
    Autorun,
    /// Asks the app to reload configuration
    Reload,
    /// Prints the current status (CLI)
    Status,
    /// Sends you notification with the current status
    Notify,
    /// Terminates the app
    Terminate,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let socket = socket_name();

    match &cli.command {
        None => {
            if let Ok(stream) = LocalSocketStream::connect(&*socket) {
                ron::ser::to_writer(stream, &Ipc::Switch)?;
            } else {
                App::new()?.start(true)?;
            }
        }
        Some(Commands::Autorun) => {
            App::new()?.start(false)?;
        }
        Some(Commands::Reload) => {
            let stream = LocalSocketStream::connect(&*socket).map_err(|_| "App is not running")?;
            ron::ser::to_writer(stream, &Ipc::Reload)?;
        }
        Some(Commands::Status) => {
            let (_, _, status) = App::new()?.status()?;
            println!("{}", status);
        }
        Some(Commands::Notify) => {
            let stream = LocalSocketStream::connect(&*socket).map_err(|_| "App is not running")?;
            ron::ser::to_writer(stream, &Ipc::Notify)?;
        }
        Some(Commands::Terminate) => {
            let stream = LocalSocketStream::connect(&*socket).map_err(|_| "App is not running")?;
            ron::ser::to_writer(stream, &Ipc::Terminate)?;
        }
    };

    Ok(())
}
