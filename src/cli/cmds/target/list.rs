use anyhow::Result;
use clap::{Arg, ArgAction, ArgMatches, Command};

use crate::{cli::Cmd, context::Ctx};

pub fn build() -> Command {
    Command::new("list").about("list targets").arg(
        Arg::new("installed")
            .help("Only list installed targets")
            .long("installed")
            .short('i')
            .action(ArgAction::SetTrue),
    )
}

pub struct BustupTargetList;

impl Cmd for BustupTargetList {
    fn update_ctx(&self, args: &ArgMatches, ctx: &mut Ctx) -> Result<()> {
        ctx.installed = args.get_flag("installed");
        Ok(())
    }

    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        println!(
            "Listing {} targets for the {} toolchain",
            if ctx.installed { "installed" } else { "all" },
            ctx.toolchain.as_ref().unwrap(),
        );
        Ok(())
    }
}
