use crate::CliArguments;

pub fn help(cli_arguments: &CliArguments) {
    println!("Help is here");

    println!(
        "long -> {} : short -> {}",
        cli_arguments.deep_search[0], cli_arguments.deep_search[1]
    );
    println!(
        "long -> {} : short -> {}",
        cli_arguments.verbose[0], cli_arguments.verbose[1]
    );
    println!(
        "long -> {} : short -> {}",
        cli_arguments.show_hidden[0], cli_arguments.show_hidden[1]
    );
    println!(
        "long -> {} : short -> {}",
        cli_arguments.show_gitignored[0], cli_arguments.show_gitignored[1]
    );
}
