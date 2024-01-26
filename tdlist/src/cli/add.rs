use crate::app::App;

use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
pub struct Args {}

pub fn run(app: App, args: Args) -> Result<()> {
    // TODO:
    Ok(())
}
