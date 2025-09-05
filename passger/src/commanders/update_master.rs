use jli::core as JLI;

pub fn update_master_commander(command_args: Result<JLI::UpdateMasterCommand, String>) {
    JLI::show_command_title("Update master password");

    let command = JLI::handle_command_err(command_args);

    match command {
        JLI::UpdateMasterCommand::Help => JLI::show_update_master_command_help(),
        JLI::UpdateMasterCommand::Update => {
            println!("Updating master password");
        }
    }
}
