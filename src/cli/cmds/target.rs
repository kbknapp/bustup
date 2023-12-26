mod add;
mod list;
mod remove;

use clap::{Args, Subcommand};

/// manage targets
#[derive(Clone, Args)]
pub struct BustupTarget {
    /// toolchain to update
    #[arg(short, long, default_value = "default")]
    pub toolchain: String,

    #[command(subcommand)]
    pub cmd: BustupTargetCmd,
}

#[derive(Clone, Subcommand)]
pub enum BustupTargetCmd {
    Add(add::BustupTargetAdd),
    List(list::BustupTargetList),
    Remove(remove::BustupTargetRemove),
}
