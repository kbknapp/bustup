use anyhow::Result;
use clap::{Arg, ArgAction, ArgMatches, Command};

use crate::{cli::Cmd, context::Ctx};

pub fn build() -> Command {
    Command::new("update")
        .about("update toolchains")
        .arg(
            Arg::new("toolchain")
                .help("toolchain to update")
                .action(ArgAction::Set)
                .default_value("default"),
        )
        .arg(
            Arg::new("force")
                .short('f')
                .long("force")
                .help("Forcibly update")
                .action(ArgAction::SetTrue),
        )
}

pub struct BustupUpdate;

impl Cmd for BustupUpdate {
    fn update_ctx(&self, args: &ArgMatches, ctx: &mut Ctx) -> Result<()> {
        ctx.toolchain = args.get_one::<String>("toolchain").cloned();
        ctx.force = args.get_flag("force");
        Ok(())
    }

    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        println!(
            "Updating {} toolchain...{}",
            ctx.toolchain.as_ref().unwrap(),
            if ctx.force { " (forced)" } else { "" }
        );
        Ok(())
    }
}
