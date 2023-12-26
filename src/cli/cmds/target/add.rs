use anyhow::Result;
use clap::Args;

use crate::{cli::Cmd, context::Ctx};

/// list targets
#[derive(Clone, Args)]
pub struct BustupTargetAdd {
    /// target to add
    #[arg(default_value = "default")]
    pub target: String,
}

impl Cmd for BustupTargetAdd {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        println!(
            "Adding the {} target to the {} toolchain",
            self.target,
            ctx.target_toolchain.as_ref().unwrap(),
        );
        Ok(())
    }
}
