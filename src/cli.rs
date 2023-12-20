use clap::{Arg, ArgAction, Command};

pub fn build() -> Command {
    Command::new("bustup")
        .about("Not rustup")
        .subcommand(
            Command::new("update")
                .about("update toolchains")
                .arg(
                    Arg::new("toolchain")
                        .help("toolchain to update")
                        .action(ArgAction::Set)
                        .default_value("default"),
                )
                .arg(
                    Arg::new("force")
                        .short('f')
                        .long("force")
                        .help("Forcibly update")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("target")
                .about("manage targets")
                .arg(
                    Arg::new("toolchain")
                        .help("toolchain to use")
                        .long("toolchain")
                        .action(ArgAction::Set)
                        .default_value("default")
                        .short('t')
                        .global(true),
                )
                .subcommand(
                    Command::new("add")
                        .about("add a target")
                        .arg(Arg::new("target").help("target to add")),
                )
                .subcommand(
                    Command::new("list").about("list targets").arg(
                        Arg::new("installed")
                            .help("Only list installed targets")
                            .long("installed")
                            .short('i')
                            .action(ArgAction::SetTrue),
                    ),
                )
                .subcommand(
                    Command::new("remove").about("remove a target").arg(
                        Arg::new("target")
                            .help("target to remove")
                            .action(ArgAction::Set),
                    ),
                ),
        )
}
