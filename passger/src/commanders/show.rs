use jli::core as JLI;

pub fn show_commander(command_args: Result<JLI::ShowCommand, String>) {
    JLI::show_command_title("Show a passkey");

    let command = JLI::handle_command_err(command_args);

    match command {
        JLI::ShowCommand::Help => JLI::show_show_command_help(),
        JLI::ShowCommand::Show { id } => {
            println!("Id: {}", id);
        }
    }
}
