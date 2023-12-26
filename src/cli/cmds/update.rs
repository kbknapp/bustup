use clap::Args;

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
