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
    https: Vec<String>,
    completions: Vec<String>,
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
        https: vec![String::from("--https"), String::from("-hs")],
        completions: vec![String::from("--completions"), String::from("-c")],
    })
});

fn generate_completions(target_shell: &str) {
    let app_name = std::env::args()
        .next()
        .and_then(|p| {
            std::path::Path::new(&p)
                .file_stem()
                .map(|s| s.to_string_lossy().into_owned())
        })
        .unwrap_or_else(|| String::from("blink"));

    let args = CLI_ARGUMENTS.lock().unwrap();
    let cmds = BLINK_COMMANDS.lock().unwrap();

    let all_flags: Vec<(&Vec<String>, &str)> = vec![
        (&args.deep_search, "Enable deep search"),
        (&args.verbose, "Enable verbose output"),
        (&args.show_hidden, "Show hidden files"),
        (&args.show_gitignored, "Show gitignored files"),
        (&cmds.help, "Show help information"),
        (&cmds.https, "Enable HTTPS mode"),
        (&cmds.completions, "Generate shell completion script"),
    ];

    match target_shell.to_lowercase().as_str() {
        "fish" => {
            for (flags, description) in all_flags {
                let mut long_flag = None;
                let mut short_flag = None;

                for flag in flags {
                    if let Some(l) = flag.strip_prefix("--") {
                        long_flag = Some(l);
                    } else if let Some(s) = flag.strip_prefix('-') {
                        short_flag = Some(s);
                    }
                }

                let mut line = format!("complete -c {app_name}");
                if let Some(l) = long_flag {
                    line.push_str(&format!(" -l {l}"));
                }
                if let Some(s) = short_flag {
                    line.push_str(&format!(" -s {s}"));
                }
                line.push_str(&format!(" -d \"{description}\""));
                println!("{line}");
            }
        }
        "bash" => {
            let mut words = Vec::new();
            for (flags, _) in all_flags {
                for flag in flags {
                    words.push(flag.as_str());
                }
            }
            println!("complete -W \"{}\" {app_name}", words.join(" "));
        }
        "zsh" => {
            println!("#compdef {app_name}");
            println!("_arguments -s \\");
            let mut specs = Vec::new();
            for (flags, description) in all_flags {
                for flag in flags {
                    specs.push(format!("  '{flag}[{description}]'"));
                }
            }
            println!("{}", specs.join(" \\\n"));
        }
        "powershell" | "ps1" => {
            let mut words = Vec::new();
            for (flags, _) in all_flags {
                for flag in flags {
                    words.push(format!("'{flag}'"));
                }
            }
            println!(
                "Register-ArgumentCompleter -Native -CommandName {app_name} -ScriptBlock {{\n\
                 \tparam($wordToComplete, $commandAst, $cursorPosition)\n\
                 \t@({})\n\
                 \t| Where-Object {{ $_ -like \"$wordToComplete*\" }}\n\
                 \t| Foreach-Object {{ [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterName', $_) }}\n\
                 }}",
                words.join(", ")
            );
        }
        _ => {
            eprintln!("Unsupported shell: {target_shell}. Available: fish, bash, zsh, powershell");
        }
    }
}

fn main() {
    let mut args = std::env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            cmd if BLINK_COMMANDS.lock().unwrap().help.iter().any(|h| h == cmd) => {
                cli::help();
            }
            cmd if BLINK_COMMANDS
                .lock()
                .unwrap()
                .completions
                .iter()
                .any(|c| c == cmd) =>
            {
                let target_shell = args.next().unwrap_or_else(|| String::from("fish"));
                generate_completions(&target_shell);
            }
            _ => { /* continue; */ }
        }
    }

    return;
}
