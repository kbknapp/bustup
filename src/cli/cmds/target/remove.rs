use clap::Args;

/// remove a target
#[derive(Clone, Args)]
pub struct BustupTargetRemove {
    /// target to remove
    #[arg(default_value = "default")]
    pub target: String,
}
