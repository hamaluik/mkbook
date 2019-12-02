use clap::{App, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(SubCommand::with_name("init")
            .about("initialize a mkbook directory tree")
            .arg(Arg::with_name("directory")
                .short("d")
                .long("directory")
                .default_value("src")
                .help("an optional directory to initialize into")
            )
        )
        .subcommand(SubCommand::with_name("build")
            .about("build the book")
            .arg(Arg::with_name("in")
                .short("i")
                .long("in")
                .default_value("src")
                .help("an optional directory to take the book sources from")
            )
            .arg(Arg::with_name("out")
                .short("o")
                .long("out")
                .default_value("book")
                .help("an optional directory to render the contents into")
            )
        )
        .subcommand(SubCommand::with_name("watch")
            .about("build the book and continually rebuild whenever the source changes")
            .arg(Arg::with_name("in")
                .short("i")
                .long("in")
                .default_value("src")
                .help("an optional directory to take the book sources from")
            )
            .arg(Arg::with_name("out")
                .short("o")
                .long("out")
                .default_value("book")
                .help("an optional directory to render the contents into")
            )
            .arg(Arg::with_name("reload")
                .short("r")
                .long("reload")
                .help("inject live-reload code so that the page automatically reloads on regeneration")
            )
        )
}
