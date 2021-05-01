use {
    clap::{crate_version, App, AppSettings, Arg},
    log::debug,
    pretty_env_logger, repl_deploy as lib,
};

const EXAMPLES: &str = "EXAMPLES:
    repl.deploy --standalone node index.js
    repl.deploy --standalone cargo run
    repl.deploy node server.js
";

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let matches = App::new("repl.deploy")
        .setting(AppSettings::TrailingVarArg)
        .bin_name("repl.deploy")
        .version(crate_version!())
        .author("Khushraj Rathod <khushraj.rathod@gmail.com>")
        .about("Automatically deploy from GitHub to Replit, lightning fast ⚡️")
        .arg(
            Arg::with_name("standalone")
                .long("standalone")
                .short("s")
                .takes_value(false)
                .help("Start an HTTP server to listen for refresh events"),
        )
        .arg(
            Arg::with_name("command")
                .multiple(true)
                .required(true)
                .help("Command to run your program"),
        )
        .after_help(EXAMPLES)
        .get_matches();

    let event_handler = if matches.is_present("standalone") {
        lib::EventHandler::Http
    } else {
        lib::EventHandler::Stdio
    };

    let mut cmd_and_args = matches.values_of("command").unwrap();
    let cmd = cmd_and_args.next().unwrap().to_owned();
    let args: Vec<String> = cmd_and_args.map(String::from).collect();

    drop(matches);

    debug!("Cmd: {:?}", cmd);
    debug!("Args: {:?}", args);
    debug!(
        "Event handler: {:?}",
        match event_handler {
            lib::EventHandler::Http => "HTTP",
            lib::EventHandler::Stdio => "STDIO",
        }
    );

    lib::listen(event_handler, cmd, args).await;
}
