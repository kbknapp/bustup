use anyhow::Result;
use clap::Args;

use crate::{cli::Cmd, context::Ctx};

/// remove a target
#[derive(Clone, Args)]
pub struct BustupTargetRemove {
    /// target to remove
    #[arg(default_value = "default")]
    pub target: String,
}

impl Cmd for BustupTargetRemove {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        println!(
            "Removing the {} target from the {} toolchain",
            self.target,
            ctx.target_toolchain.as_ref().unwrap(),
        );
        Ok(())
    }
}
