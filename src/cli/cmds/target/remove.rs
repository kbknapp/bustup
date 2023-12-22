use anyhow::Result;
use clap::{Arg, ArgAction, ArgMatches, Command};

use crate::{cli::Cmd, context::Ctx};

pub fn build() -> Command {
    Command::new("remove").about("remove a target").arg(
        Arg::new("target")
            .help("target to remove")
            .action(ArgAction::Set),
    )
}

pub struct BustupTargetRemove;

impl Cmd for BustupTargetRemove {
    fn update_ctx(&self, args: &ArgMatches, ctx: &mut Ctx) -> Result<()> {
        ctx.target = args.get_one::<String>("target").cloned();
        Ok(())
    }

    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        println!(
            "Removing the {} target from the {} toolchain",
            ctx.target.as_ref().unwrap(),
            ctx.toolchain.as_ref().unwrap(),
        );
        Ok(())
    }
}
