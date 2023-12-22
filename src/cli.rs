pub mod cmds;

use anyhow::Result;
use clap::{ArgMatches, Command};

use crate::context::Ctx;

pub trait Cmd {
    fn update_ctx(&self, _args: &ArgMatches, _ctx: &mut Ctx) -> Result<()> {
        Ok(())
    }
    fn run(&self, _ctx: &mut Ctx) -> Result<()> {
        Ok(())
    }
    fn next_cmd<'a>(&self, _args: &'a ArgMatches) -> Option<(Box<dyn Cmd>, &'a ArgMatches)> {
        None
    }
}

impl dyn Cmd + 'static {
    pub fn walk_exec(&self, args: &ArgMatches, ctx: &mut Ctx) -> Result<()> {
        self.update_ctx(args, ctx)?;
        self.run(ctx)?;
        if let Some((c, m)) = self.next_cmd(args) {
            return c.walk_exec(m, ctx);
        }
        Ok(())
    }
}

pub struct Bustup;

impl Cmd for Bustup {
    fn next_cmd<'a>(&self, args: &'a ArgMatches) -> Option<(Box<dyn Cmd>, &'a ArgMatches)> {
        match args.subcommand() {
            Some(("update", m)) => Some((Box::new(cmds::update::BustupUpdate {}), m)),
            Some(("target", m)) => Some((Box::new(cmds::target::BustupTarget {}), m)),
            _ => None,
        }
    }
}

pub fn build() -> Command {
    Command::new("bustup")
        .about("Not rustup")
        .subcommand(cmds::update::build())
        .subcommand(cmds::target::build())
}
