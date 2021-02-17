use std::process::{exit, Command};
use human_panic::setup_panic;
use gitx::{Result};

fn main() {
    setup_panic!();

    if let Err(e) = app() {
        eprintln!("{}", e);
        exit(1);
    }
}

fn app() -> Result<()> {
    let mut cmd = Command::new("git");
    cmd.arg("status");

    let output = cmd.output()?;

    println!("{:?}", output);

    let status = cmd.status()?;

    if !status.success() {
        return Err("something went wrong composing entry, please try again".into());
    }

    Ok(())
}
