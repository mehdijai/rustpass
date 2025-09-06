use jli::core as JLI;

pub fn add_commander(command: JLI::AddCommand) {
    JLI::show_command_title("Register new passkey");

    match command {
        JLI::AddCommand::Help => JLI::show_add_command_help(),
        JLI::AddCommand::Add { name, email } => {
            println!("Name: {}", name);
            println!("Email: {}", email);
        }
    }
}
