use crate::{BlinkCommands, CliArguments};

pub fn help(cli_arguments: &CliArguments, blink_commands: &BlinkCommands) {
    println!("Ametrine Foundation: blink");
    println!("Authors -> Noticxs, Ametrine Foundation\n");

    println!("blink help:\n");

    println!("Commands:");
    println!(
        "    {} or {}              : shows this help command",
        blink_commands.help[0], blink_commands.help[1]
    );

    print!("\n");

    println!("Arguments:");

    println!(
        "    {} or {}              : enables deep searching (search through other directories)",
        cli_arguments.deep_search[0], cli_arguments.deep_search[1]
    );
    println!(
        "    {} or {}           : enables verbose debug output (see everything blink is doing)",
        cli_arguments.verbose[0], cli_arguments.verbose[1]
    );
    println!(
        "    {} or {}      : show files that are hidden due to the '.' prefix",
        cli_arguments.show_hidden[0], cli_arguments.show_hidden[1]
    );
    println!(
        "    {} or {}  : show files hidden by .gitignore",
        cli_arguments.show_gitignored[0], cli_arguments.show_gitignored[1]
    );
}
