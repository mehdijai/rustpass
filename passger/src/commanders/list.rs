use jli::core as JLI;

pub fn list_commander(command_args: Result<JLI::ListCommand, String>) {
    JLI::show_command_title("List of passkeys");

    let command = JLI::handle_command_err(command_args);

    match command {
        JLI::ListCommand::Help => JLI::show_list_command_help(),
        JLI::ListCommand::List => {
            println!("Listing the vault");
        }
    }
}
