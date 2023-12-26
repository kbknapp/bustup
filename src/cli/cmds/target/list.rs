use clap::Args;

/// list targets
#[derive(Clone, Args)]
pub struct BustupTargetList {
    /// only list installed targets
    #[arg(short, long)]
    pub installed: bool,
}
