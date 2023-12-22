mod cli;
mod context;

use crate::{
    cli::{Bustup, Cmd},
    context::Ctx,
};

fn main() -> anyhow::Result<()> {
    let args = cli::build().get_matches();

    let mut ctx = Ctx::default();

    let cmd: Box<dyn Cmd> = Box::new(Bustup);

    cmd.walk_exec(&args, &mut ctx)
}
