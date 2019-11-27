use clap::{App, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(SubCommand::with_name("init")
            .about("initialized the directory structi")
        )
        .subcommand(SubCommand::with_name("init")
            .about("initialize a mkbook directory tree")
            .arg(Arg::with_name("directory")
                .help("an optional directory to initialize into (defaults to the CWD)")
            )
        )
        .subcommand(SubCommand::with_name("build")
            .about("build the book")
            .arg(Arg::with_name("directory")
                .help("an optional directory to build the book in (defaults to the CWD)")
            )
        )
}
