mod cli;

use anyhow::Result;
use clap::Parser;

use crate::cli::Bustup;

fn main() -> Result<()> {
    let args = Bustup::parse();

    todo!("Implement bustup");

    Ok(())
}
