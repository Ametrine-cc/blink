mod cli;

struct BlinkCommands {
    help: Vec<String>,
}

struct CliArguments {
    deep_search: Vec<String>,
    verbose: Vec<String>,
    show_hidden: Vec<String>,
    show_gitignored: Vec<String>,
}

fn main() {
    let cli_arguments = CliArguments {
        deep_search: vec![String::from("--deep"), String::from("-d")],
        verbose: vec![String::from("--verbose"), String::from("-v")],
        show_hidden: vec![String::from("--show-hidden"), String::from("-sh")],
        show_gitignored: vec![String::from("--show-gitignored"), String::from("-sg")],
    };

    let blink_commands = BlinkCommands {
        help: vec![String::from("--help"), String::from("-h")],
    };

    let mut args = std::env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            cmd if blink_commands.help.iter().any(|h| h == cmd) => {
                cli::help(&cli_arguments, &blink_commands);
            }
            _ => { /* continue; */ }
        }
    }

    return;
}
