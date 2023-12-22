mod add;
mod list;
mod remove;

use anyhow::Result;
use clap::{Arg, ArgAction, ArgMatches, Command};

use crate::{cli::Cmd, context::Ctx};

pub fn build() -> Command {
    Command::new("target")
        .about("manage targets")
        .arg(
            Arg::new("toolchain")
                .help("toolchain to use")
                .long("toolchain")
                .action(ArgAction::Set)
                .default_value("default")
                .short('t')
                .global(true),
        )
        .subcommand(add::build())
        .subcommand(list::build())
        .subcommand(remove::build())
}

pub struct BustupTarget;

impl Cmd for BustupTarget {
    fn update_ctx(&self, args: &ArgMatches, ctx: &mut Ctx) -> Result<()> {
        ctx.toolchain = args.get_one::<String>("toolchain").cloned();
        Ok(())
    }

    fn next_cmd<'a>(&self, matches: &'a ArgMatches) -> Option<(Box<dyn Cmd>, &'a ArgMatches)> {
        match matches.subcommand() {
            Some(("add", m)) => Some((Box::new(add::BustupTargetAdd), m)),
            Some(("list", m)) => Some((Box::new(list::BustupTargetList), m)),
            Some(("remove", m)) => Some((Box::new(remove::BustupTargetRemove), m)),
            _ => None,
        }
    }
}
