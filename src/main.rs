#![windows_subsystem = "windows"]

use clap::{Parser, Subcommand};
use interprocess::local_socket::LocalSocketStream;

mod activities;
mod app;
mod math;

use app::{socket_name, App, Ipc};

/// Work-break balancer can track your work time and suggest break time
#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Asks the app to reload configuration
    Reload,
    /// Sends you notification with the current status
    Notify,
    /// Switches between work and break time
    Switch,
    /// Terminates the app
    Terminate,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let socket = socket_name();

    match &cli.command {
        Some(Commands::Reload) => {
            let stream = LocalSocketStream::connect(&*socket).map_err(|_| "App is not running")?;
            ron::ser::to_writer(stream, &Ipc::Reload)?;
        }
        Some(Commands::Notify) => {
            let stream = LocalSocketStream::connect(&*socket).map_err(|_| "App is not running")?;
            ron::ser::to_writer(stream, &Ipc::Notify)?;
        }
        Some(Commands::Switch) => {
            let stream = LocalSocketStream::connect(&*socket).map_err(|_| "App is not running")?;
            ron::ser::to_writer(stream, &Ipc::Switch)?;
        }
        Some(Commands::Terminate) => {
            let stream = LocalSocketStream::connect(&*socket).map_err(|_| "App is not running")?;
            ron::ser::to_writer(stream, &Ipc::Terminate)?;
        }
        None => {
            let mut app = App::new()?;
            if let Ok(stream) = LocalSocketStream::connect(&*socket) {
                ron::ser::to_writer(stream, &Ipc::Update)?;
                let (_, _, status) = app.status()?;

                println!("{}", status);
            } else {
                app.start()?;
            }
        }
    };

    Ok(())
}
