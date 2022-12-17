use clap::Parser;

#[derive(Parser)]
pub struct Args {
    pub toggle: bool,
    pub bind_plugin_id: Option<i32>,
}
