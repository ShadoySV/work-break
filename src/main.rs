#![windows_subsystem = "windows"]

use std::io::Write;

use clap::{Parser, Subcommand};
use interprocess::local_socket::{
    prelude::*, traits::Stream, GenericFilePath, GenericNamespaced, NameType, ToFsName, ToNsName,
};

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
    /// Tracks work time
    Work,
    /// Tracks break time
    Break,
    /// Terminates the app
    Terminate,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let socket = socket_name();
    let socket = if GenericNamespaced::is_supported() {
        format!("{socket}.sock").to_ns_name::<GenericNamespaced>()?
    } else {
        format!("/tmp/{socket}.sock").to_fs_name::<GenericFilePath>()?
    };

    match &cli.command {
        None => {
            if let Ok(mut stream) = <LocalSocketStream as Stream>::connect(socket) {
                let s = ron::ser::to_string(&Ipc::Switch)?;
                stream.write_all(s.as_bytes())?;
            } else {
                App::new()?.start(true, None)?;
            }
        }
        Some(Commands::Work) => {
            if let Ok(mut stream) = <LocalSocketStream as Stream>::connect(socket) {
                let s = ron::ser::to_string(&Ipc::Work)?;
                stream.write_all(s.as_bytes())?;
            } else {
                App::new()?.start(true, Some(true))?;
            }
        }
        Some(Commands::Break) => {
            if let Ok(mut stream) = <LocalSocketStream as Stream>::connect(socket) {
                let s = ron::ser::to_string(&Ipc::Break)?;
                stream.write_all(s.as_bytes())?;
            } else {
                App::new()?.start(true, Some(false))?;
            }
        }
        Some(Commands::Autorun) => {
            App::new()?.trancate_activities().start(false, None)?;
        }
        Some(Commands::Reload) => {
            if let Ok(mut stream) = <LocalSocketStream as Stream>::connect(socket) {
                let s = ron::ser::to_string(&Ipc::Reload)?;
                stream.write_all(s.as_bytes())?;
            } else {
                Err("App is not running")?;
            }
        }
        Some(Commands::Status) => {
            let (_, _, status) = App::new()?.status()?;
            println!("{}", status);
        }
        Some(Commands::Notify) => {
            if let Ok(mut stream) = <LocalSocketStream as Stream>::connect(socket) {
                let s = ron::ser::to_string(&Ipc::Notify)?;
                stream.write_all(s.as_bytes())?;
            } else {
                Err("App is not running")?;
            }
        }
        Some(Commands::Terminate) => {
            if let Ok(mut stream) = <LocalSocketStream as Stream>::connect(socket) {
                let s = ron::ser::to_string(&Ipc::Terminate)?;
                stream.write_all(s.as_bytes())?;
            } else {
                Err("App is not running")?;
            }
        }
    };

    Ok(())
}
