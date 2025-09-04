pub fn show_welcome() {
    print!("{esc}c", esc = 27 as char);
    println!("====================================================");
    println!("==============| JLI Password Manager |==============");
    println!("====================================================");
    println!("");
}

pub fn show_help() {
    let options = vec![
        ("--help", "Show this help message"),
        ("--version", "Show version information"),
    ];
    let commands = vec![
        ("init", "Initialize a new password database"),
        ("create", "Interactive password creation wizard"),
        ("add", "Add a new password entry"),
        ("list", "Show all password entries (public info only)"),
        ("show", "Display a specific entry and copy password"),
        ("delete", "Remove a password entry"),
        ("update", "Update an existing password entry"),
        ("update-master", "Update the master password"),
    ];
    // Password Manager CLI

    println!("");
    println!("USAGE:");
    println!("");
    println!("\tpassger [OPTIONS] [COMMAND]");
    println!("");
    println!("OPTIONS:");
    println!("");

    for (option, description) in options {
        println!("\t{}\t\t\t{}", option, description);
    }

    println!("");
    println!("COMMANDS:");
    println!("");
    for (command, description) in commands {
        println!("\t{}\t\t\t{}", command, description);
    }

    println!("");
    println!("Use 'passger [COMMAND] --help' for more information on a specific command.");
    println!("");
}

pub fn show_version() {
    println!("Version: 1.0.0");
    println!("");
}
