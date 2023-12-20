mod cli;

fn main() -> anyhow::Result<()> {
    let args = cli::build().get_matches();

    todo!("Run the program!");

    Ok(())
}
