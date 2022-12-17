use std::process::Command;

fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=icons");
    Command::new("./install_icons.sh").spawn()?;
    Ok(())
}
