use crate::{BLINK_COMMANDS, CLI_ARGUMENTS};

pub fn help() {
    let cmd = BLINK_COMMANDS.lock().unwrap();
    let args = CLI_ARGUMENTS.lock().unwrap();

    println!("Ametrine Foundation: blink");
    println!("Authors -> Noticxs, Ametrine Foundation\n");

    println!("blink help:\n");

    println!("Commands:");
    println!(
        "\t{} or {}              : shows this help command",
        cmd.help[0], cmd.help[1]
    );

    print!("\n");

    println!("Arguments:");

    println!(
        "\t{} or {}              : enables deep searching (search through other directories)",
        args.deep_search[0], args.deep_search[1]
    );
    println!(
        "\t{} or {}           : enables verbose debug output (see everything blink is doing)",
        args.verbose[0], args.verbose[1]
    );
    println!(
        "\t{} or {}      : show files that are hidden due to the '.' prefix",
        args.show_hidden[0], args.show_hidden[1]
    );
    println!(
        "\t{} or {}  : show files hidden by .gitignore",
        args.show_gitignored[0], args.show_gitignored[1]
    );
}
