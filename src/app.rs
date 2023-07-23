use std::{
    sync::mpsc::sync_channel,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use chrono::{DateTime, Local};
use interprocess::local_socket::LocalSocketListener;
use notify_rust::{Notification, Urgency};
use serde::{Deserialize, Serialize};

use crate::{
    activities::Activities,
    math::{CoefficientA, CoefficientB, CoefficientC, Formula},
};

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");

#[cfg(not(debug_assertions))]
pub const APP_UUID: &str = "19ef189a-0474-44a8-8168-530cc14ccacc";

#[cfg(debug_assertions)]
pub const APP_UUID: &str = "735d31a3-bfb0-47f9-9ce6-647202507738";

pub fn socket_name() -> String {
    format!("@{}_{}", APP_NAME, APP_UUID)
}

#[cfg(debug_assertions)]
pub const CONFIG_NAME: Option<&str> = Some("dev-config");

#[cfg(not(debug_assertions))]
pub const CONFIG_NAME: Option<&str> = None;

#[cfg(debug_assertions)]
pub const STATE_NAME: Option<&str> = Some("dev-state");

#[cfg(not(debug_assertions))]
pub const STATE_NAME: Option<&str> = Some("default-state");

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub coefficient_a: CoefficientA,
    #[serde(default)]
    pub coefficient_b: CoefficientB,
    #[serde(default)]
    pub coefficient_c: CoefficientC,
    #[serde(default)]
    pub daily_work_time_limit: u16,
    #[serde(default)]
    pub work_days_start_at: u8,
}
impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(confy::load(APP_NAME, CONFIG_NAME)?)
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct State {
    #[serde(default)]
    pub activities: Activities,
}

#[derive(Serialize, Deserialize)]
pub enum Ipc {
    Reload,
    Update,
    Notify,
    Switch,
    Terminate,
}

pub struct App {
    last_strain: Duration,
    last_work: Duration,
    pub config: Config,
    state: State,
}

impl App {
    pub fn new() -> Result<App, Box<dyn std::error::Error>> {
        let config: Config = confy::load(APP_NAME, CONFIG_NAME)?;
        let state: State = confy::load(APP_NAME, STATE_NAME)?;
        let formula = Formula::new(
            &config.coefficient_a,
            &config.coefficient_b,
            &config.coefficient_c,
        );
        let now = SystemTime::now();
        let (_, last_strain, last_work) = state.activities.work(&formula, now);
        Ok(App {
            last_strain,
            last_work,
            config,
            state,
        })
    }

    pub fn status(&mut self) -> Result<(Duration, Duration, String), Box<dyn std::error::Error>> {
        let formula = Formula::new(
            &self.config.coefficient_a,
            &self.config.coefficient_b,
            &self.config.coefficient_c,
        );
        let now = SystemTime::now();
        let now_ch: DateTime<Local> = DateTime::from(now);
        let morning = now_ch
            .date_naive()
            .and_hms_opt(self.config.work_days_start_at.into(), 0, 0)
            .unwrap();
        let mut morning_local = morning.and_local_timezone(Local).unwrap();
        if now_ch < morning_local {
            morning_local = morning_local
                .checked_sub_days(chrono::Days::new(1))
                .unwrap();
        }
        let truncate_point = UNIX_EPOCH + Duration::from_secs(morning_local.timestamp() as u64);
        self.state.activities.truncate_until(truncate_point);

        let (end, strain, work) = self.state.activities.work(&formula, now);

        let phase = if !self.state.activities.list.is_empty() && end.is_none() {
            match strain.as_secs() / 60 {
                0..=24 => "Pomodoro",
                25..=51 => "Efficiency",
                52.. => "Injury",
            }
        } else if strain.is_zero() {
            "Ready"
        } else {
            "Break"
        };

        let status = format!(
            "Phase: {phase}\nStrain: {} min.\nBreak: {} min, ends at: {}\nToday: {} hrs, {} min.",
            strain.as_secs() / 60,
            formula.compute_break(strain).as_secs() / 60,
            DateTime::<Local>::from(now + formula.compute_break(strain)).format("%X"),
            work.as_secs() / 3600,
            (work.as_secs() - work.as_secs() / 3600 * 3600) / 60
        );

        Ok((strain, work, status))
    }

    pub fn notify(&mut self, notify_anyway: bool) -> Result<(), Box<dyn std::error::Error>> {
        let (strain, work, status) = self.status()?;

        const POMODORO: Duration = Duration::from_secs(25 * 60);
        const EFFICIENCY: Duration = Duration::from_secs(52 * 60);
        let daily_work_time_limit =
            Duration::from_secs(self.config.daily_work_time_limit as u64 * 60);
        let mut urgency = Urgency::Low;
        let notify_on_threshold = self.last_strain < POMODORO && strain >= POMODORO
            || self.last_strain < EFFICIENCY && strain >= EFFICIENCY
            || !self.last_strain.is_zero() && strain.is_zero()
            || !daily_work_time_limit.is_zero()
                && self.last_work < daily_work_time_limit
                && work >= daily_work_time_limit;

        if strain >= POMODORO {
            urgency = Urgency::Normal;
        }

        if strain >= EFFICIENCY || !daily_work_time_limit.is_zero() && work >= daily_work_time_limit
        {
            urgency = Urgency::Critical;
        }

        self.last_strain = strain;
        self.last_work = work;

        if notify_anyway || notify_on_threshold {
            let mut notification = Notification::new();
            notification.summary("Work-break balancer").body(&status);

            #[cfg(not(any(target_os = "windows", target_os = "macos")))]
            notification.urgency(urgency);

            #[cfg(target_os = "windows")]
            if urgency == Urgency::Critical {
                use notify_rust::Timeout;
                notification.timeout(Timeout::Never);
            };

            notification.show()?;
        }

        Ok(())
    }

    pub fn switch(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let now = SystemTime::now();
        self.state.activities.switch(now);
        confy::store(APP_NAME, STATE_NAME, &self.state)?;
        self.notify(true)?;
        Ok(())
    }

    pub fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let socket = socket_name();

        let (sender, receiver) = sync_channel(0);

        let ctrlc_sender = sender.clone();
        ctrlc::set_handler(move || {
            ctrlc_sender.send(Ipc::Terminate).unwrap();
        })?;

        let socket_sender = sender.clone();
        thread::spawn(move || {
            let stream = LocalSocketListener::bind(socket).unwrap();
            println!("App has been started");

            for stream in stream.incoming() {
                if let Ok(stream) = stream {
                    if let Ok(ipc) = ron::de::from_reader(stream) {
                        socket_sender.send(ipc).unwrap();
                    } else {
                        eprintln!("Ipc message is not recognized!")
                    }
                } else {
                    socket_sender.send(Ipc::Terminate).unwrap();
                    break;
                }
            }
        });

        thread::spawn(move || loop {
            thread::park_timeout(Duration::from_secs(60));
            sender.send(Ipc::Update).unwrap();
        });

        let mut app = App::new()?;

        for ipc in receiver.iter() {
            match ipc {
                Ipc::Reload => {
                    app.config = Config::load()?;
                }
                Ipc::Update => {
                    app.notify(false)?;
                }
                Ipc::Notify => {
                    app.notify(true)?;
                }
                Ipc::Switch => {
                    app.switch()?;
                }
                Ipc::Terminate => {
                    break;
                }
            }
        }

        Ok(())
    }
}
