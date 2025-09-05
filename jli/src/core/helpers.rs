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

pub fn show_add_command_help() {
    println!("Add a new passkey to the vault quickly.");
    println!("Usage: passger add [options]");
    println!("Options:");
    println!("  --help, -h     Show this help message and exit");
    println!("  -n             Specify the name of the new key");
    println!("  -e             Specify the email or username of the new key");
    println!("Example:");
    println!("passger add -n \"Gmail\" -e \"mygmail@gmail.com\"");
}

pub fn show_update_command_help() {
    println!("Update existing passkey.");
    println!("Usage: passger update [options]");
    println!("Options:");
    println!("  --help, -h     Show this help message and exit");
    println!("  -i, --id             Specify the ID of the key");
    println!("Example:");
    println!("passger update -i gmail");
}

pub fn show_update_details_command_help() {
    println!("Update existing passkey details; name and/or email.");
    println!("Usage: passger update-details [options]");
    println!("Options:");
    println!("  --help, -h     Show this help message and exit");
    println!("  -i, --id             Specify the ID of the key");
    println!("  -e             Specify the new email of the key");
    println!("  -n             Specify the new name of the key");
    println!("Example:");
    println!("passger update-details -i gmail");
}

pub fn show_show_command_help() {
    println!(
        "Show existing passkey by ID. The secret key won't be displayed, instead, it will be copied to your clipboard."
    );
    println!("Usage: passger show [options]");
    println!("Options:");
    println!("  --help, -h     Show this help message and exit");
    println!("  -i, --id             Specify the ID of the key");
    println!("Example:");
    println!("passger show -i gmail");
}

pub fn show_delete_command_help() {
    println!("Delete existing passkey by ID. WARNING: Irreversible action!");
    println!("Usage: passger delete [options]");
    println!("Options:");
    println!("  --help, -h     Show this help message and exit");
    println!("  -i, --id             Specify the ID of the key");
    println!("Example:");
    println!("passger delete -i gmail");
}

pub fn show_init_command_help() {
    println!(
        "Initialize a new vault. If a vault already existing, you will be asked to override it."
    );
    println!("Usage: passger init [options]");
    println!("Options:");
    println!("  --help, -h     Show this help message and exit");
    println!("Example:");
    println!("passger init");
}

pub fn show_list_command_help() {
    println!("Listing your vault as a table. Showing only public information.");
    println!("Usage: passger list [options]");
    println!("Options:");
    println!("  --help, -h     Show this help message and exit");
    println!("Example:");
    println!("passger list");
}

pub fn show_create_command_help() {
    println!("Create new passkey step by step. Just like the \"add\" command.");
    println!("Usage: passger create [options]");
    println!("Options:");
    println!("  --help, -h     Show this help message and exit");
    println!("Example:");
    println!("passger create");
}

pub fn show_update_master_command_help() {
    println!("Update the master password. This will require the old master password.");
    println!("Usage: passger update-master [options]");
    println!("Options:");
    println!("  --help, -h     Show this help message and exit");
    println!("Example:");
    println!("passger update-master");
}

pub fn show_command_title(command: &str) {
    println!("");
    println!("==============| {command} |==============");
    println!("");
}

pub fn show_footer() {
    println!("");
    println!("=====================| See ya |=====================");
    println!("");
}
