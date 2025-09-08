use jli::core as JLI;

use crate::utils::yn_user_input;

pub fn init_commander(command: JLI::InitCommand) {
    JLI::show_command_title("Initialize new vault");

    match command {
        JLI::InitCommand::Help => JLI::show_init_command_help(),
        JLI::InitCommand::Init => match db_manager::initialize_db(None) {
            Err(err) => match err {
                com::Error::AlreadyInitialized => {
                    let error_message = com::parse_error(err);
                    println!("{}", error_message);
                    println!("Do you want to overwrite the existing vault? (y/n)");

                    if yn_user_input() {
                        match db_manager::initialize_db(Some(true)) {
                            Ok(_) => println!("Vault overwritten successfully!"),
                            Err(err) => com::print_error(err),
                        }
                    }

                    println!("Initializing skipped.");
                }
                _ => {
                    com::print_error(err);
                }
            },
            Ok(_) => {
                // TODO: Handle master password registration
                println!("New vault initialized successfully!")
            }
        },
    }
}
