use jli::core as JLI;

pub fn add_commander(add_command_args: Result<JLI::AddCommand, String>) {
    JLI::show_command_title("Register new passkey");

    let command = JLI::handle_command_err(add_command_args);

    match command {
        JLI::AddCommand::Help => JLI::show_add_command_help(),
        JLI::AddCommand::Add { name, email } => {
            println!("Name: {}", name);
            println!("Email: {}", email);
        }
    }
}
