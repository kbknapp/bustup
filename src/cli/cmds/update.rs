use anyhow::Result;
use clap::Args;

use crate::{cli::Cmd, context::Ctx};

/// update toolchains
#[derive(Clone, Args)]
pub struct BustupUpdate {
    /// toolchain to update
    #[arg(default_value = "default")]
    pub toolchain: String,

    /// forcibly update toolchain
    #[arg(short, long)]
    pub force: bool,
}

impl Cmd for BustupUpdate {
    fn run(&self, _ctx: &mut Ctx) -> Result<()> {
        println!(
            "updating toolchain {}{}",
            self.toolchain,
            if self.force { " (forced)" } else { "" }
        );
        Ok(())
    }
}
