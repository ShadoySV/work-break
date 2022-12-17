use anyhow::Context;
use args::Args;
use chrono::{DateTime, Local};
use clap::Parser;
use config::Config;
use math::{work, work_break};
use std::{
    env::current_exe,
    process::Command,
    time::{Duration, SystemTime},
};

mod args;
mod config;
mod math;

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut cfg: Config = confy::load(APP_NAME, None)?;

    if let Some(plugin_id) = args.bind_plugin_id {
        cfg.plugin_id = plugin_id;
    }

    let previous_work = cfg.work;

    if cfg.is_working {
        cfg.work += cfg.at.elapsed()?;
    } else {
        cfg.work = work(work_break(cfg.work).saturating_sub(cfg.at.elapsed()?));
    }
    cfg.at = SystemTime::now();

    if args.toggle {
        cfg.is_working = !cfg.is_working;
        confy::store(APP_NAME, None, &cfg)?;

        Command::new("xfce4-panel")
            .arg(format!(
                "--plugin-event=genmon-{}:refresh:bool:true",
                cfg.plugin_id
            ))
            .spawn()?;
    } else {
        {
            let phase = if cfg.is_working {
                match cfg.work.as_secs() / 60 {
                    0..=24 => "Pomodoro",
                    25..=51 => "Efficiency",
                    52.. => "Injury",
                }
            } else if cfg.work.is_zero() {
                "Ready"
            } else {
                "Break"
            };

            println!("<icon>workbreak-{}</icon>", phase.to_lowercase());

            println!(
                "<tool>Phase: {phase}\nWork: {} min.\nBreak: {} min.\nEnds at: {}</tool>",
                cfg.work.as_secs() / 60,
                work_break(cfg.work).as_secs() / 60,
                DateTime::<Local>::from(SystemTime::now() + work_break(cfg.work)).format("%X")
            );

            println!(
                "<iconclick>{} toggle</iconclick>",
                current_exe()
                    .with_context(|| "Failed to get current executable path!".to_owned())?
                    .display()
            );
        }

        const POMODORO: Duration = Duration::from_secs(25 * 60);
        const EFFICIENCY: Duration = Duration::from_secs(52 * 60);
        if previous_work < POMODORO && cfg.work >= POMODORO
            || previous_work < EFFICIENCY && cfg.work >= EFFICIENCY
            || !previous_work.is_zero() && cfg.work.is_zero()
        {
            confy::store(APP_NAME, None, &cfg)?;
            Command::new("beep").spawn()?;
        }
    }

    Ok(())
}
