mod add;
mod list;
mod remove;

use anyhow::Result;
use clap::{Args, Subcommand};

use crate::{cli::Cmd, context::Ctx};

/// manage targets
#[derive(Clone, Args)]
pub struct BustupTarget {
    /// toolchain to update
    #[arg(short, long, default_value = "default")]
    pub toolchain: String,

    #[command(subcommand)]
    pub cmd: BustupTargetCmd,
}

#[enum_delegate::implement(Cmd)]
#[derive(Clone, Subcommand)]
pub enum BustupTargetCmd {
    Add(add::BustupTargetAdd),
    List(list::BustupTargetList),
    Remove(remove::BustupTargetRemove),
}

impl Cmd for BustupTarget {
    fn update_ctx(&self, ctx: &mut Ctx) -> Result<()> {
        ctx.target_toolchain = Some(self.toolchain.clone());
        Ok(())
    }
    fn next_cmd(&self) -> Option<&dyn Cmd> {
        Some(&self.cmd)
    }
}
