use jli::core as JLI;

pub fn list_commander(command: JLI::ListCommand) {
    JLI::show_command_title("List of passkeys");

    match command {
        JLI::ListCommand::Help => JLI::show_list_command_help(),
        JLI::ListCommand::List => {
            println!("Listing the vault");
        }
    }
}
