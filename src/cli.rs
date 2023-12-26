mod cmds;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::{cli::cmds::*, context::Ctx};

/// not rustup
#[derive(Parser)]
pub struct Bustup {
    #[command(subcommand)]
    pub cmd: BustupCmd,
}

#[enum_delegate::implement(Cmd)]
#[derive(Clone, Subcommand)]
pub enum BustupCmd {
    Update(update::BustupUpdate),
    Target(target::BustupTarget),
}

impl Cmd for Bustup {
    fn next_cmd(&self) -> Option<&dyn Cmd> {
        Some(&self.cmd)
    }
}

#[enum_delegate::register]
pub trait Cmd {
    fn update_ctx(&self, _ctx: &mut Ctx) -> Result<()> {
        Ok(())
    }

    fn run(&self, _ctx: &mut Ctx) -> Result<()> {
        Ok(())
    }

    fn next_cmd(&self) -> Option<&dyn Cmd> {
        None
    }
}

impl<'a> dyn Cmd + 'a {
    pub fn walk_exec(&self, ctx: &mut Ctx) -> Result<()> {
        self.update_ctx(ctx)?;
        self.run(ctx)?;
        if let Some(next) = self.next_cmd() {
            return next.walk_exec(ctx);
        }

        Ok(())
    }
}
