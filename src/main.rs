mod cli;
mod context;

use anyhow::Result;
use clap::Parser;

use crate::cli::{Bustup, Cmd};

fn main() -> Result<()> {
    let args = Bustup::parse();

    let mut ctx = context::Ctx::default();
    let args: &dyn Cmd = &args;
    args.walk_exec(&mut ctx)
}
