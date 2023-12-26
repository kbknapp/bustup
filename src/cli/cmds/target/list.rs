use anyhow::Result;
use clap::Args;

use crate::{cli::Cmd, context::Ctx};

/// list targets
#[derive(Clone, Args)]
pub struct BustupTargetList {
    /// only list installed targets
    #[arg(short, long)]
    pub installed: bool,
}

impl Cmd for BustupTargetList {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        println!(
            "Listing {} targets for the {} toolchain",
            if self.installed { "installed" } else { "all" },
            ctx.target_toolchain.as_ref().unwrap(),
        );
        Ok(())
    }
}
