#[derive(Default)]
pub struct Ctx {
    pub toolchain: Option<String>,
    pub target: Option<String>,
    pub installed: bool,
    pub force: bool,
}
