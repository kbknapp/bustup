use clap::Args;

/// list targets
#[derive(Clone, Args)]
pub struct BustupTargetAdd {
    /// target to add
    #[arg(default_value = "default")]
    pub target: String,
}
