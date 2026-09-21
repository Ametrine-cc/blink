use std::sync::{LazyLock, Mutex};

mod cli;

struct Arguments {
    deep_search: Vec<String>,
    verbose: Vec<String>,
    show_hidden: Vec<String>,
    show_gitignored: Vec<String>,
}

struct Commands {
    help: Vec<String>,
}

static CLI_ARGUMENTS: LazyLock<Mutex<Arguments>> = LazyLock::new(|| {
    Mutex::new(Arguments {
        deep_search: vec![String::from("--deep"), String::from("-d")],
        verbose: vec![String::from("--verbose"), String::from("-v")],
        show_hidden: vec![String::from("--show-hidden"), String::from("-sh")],
        show_gitignored: vec![String::from("--show-gitignored"), String::from("-sg")],
    })
});

static BLINK_COMMANDS: LazyLock<Mutex<Commands>> = LazyLock::new(|| {
    Mutex::new(Commands {
        help: vec![String::from("--help"), String::from("-h")],
    })
});

fn main() {
    let mut args = std::env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            cmd if BLINK_COMMANDS.lock().unwrap().help.iter().any(|h| h == cmd) => {
                cli::help();
            }
            _ => { /* continue; */ }
        }
    }

    return;
}
