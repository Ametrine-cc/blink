use crate::{BLINK_COMMANDS, CLI_ARGUMENTS};

pub fn help() {
    let cmds = BLINK_COMMANDS.lock().unwrap();
    let args = CLI_ARGUMENTS.lock().unwrap();

    println!("Ametrine Foundation: blink");
    println!("Authors -> Noticxs, Ametrine Foundation\n");

    let cmds_entries = [
        (
            format!("    {} or {}", cmds.help[0], cmds.help[1]),
            "shows this help command",
        ),
        (
            format!("    {} or {}", cmds.https[0], cmds.https[1]),
            "allows searching over the internet",
        ),
        (
            format!(
                "    {} or {} <shell_name>",
                cmds.completions[0], cmds.completions[1]
            ),
            "add shell completions for blink",
        ),
    ];

    let args_entries = [
        (
            format!("    {} or {}", args.deep_search[0], args.deep_search[1]),
            "enables deep searching (search through other files)",
        ),
        (
            format!("    {} or {}", args.verbose[0], args.verbose[1]),
            "enables verbose debug output (see everything)",
        ),
        (
            format!("    {} or {}", args.show_hidden[0], args.show_hidden[1]),
            "show files that are hidden due to the '.' prefix",
        ),
        (
            format!(
                "    {} or {}",
                args.show_gitignored[0], args.show_gitignored[1]
            ),
            "show files hidden by .gitignore",
        ),
    ];

    let max_len = args_entries
        .iter()
        .map(|(cmd, _)| cmd.len())
        .max()
        .unwrap_or(0);

    println!("blink commands:");
    for (cmd, desc) in &cmds_entries {
        println!("{:<width$} : {}", cmd, desc, width = max_len);
    }

    print!("\n");

    println!("blink arguments:");
    for (cmd, desc) in &args_entries {
        println!("{:<width$} : {}", cmd, desc, width = max_len);
    }
}
