use jli::core as JLI;

pub fn update_master_commander(command: JLI::UpdateMasterCommand) {
    JLI::show_command_title("Update master password");

    match command {
        JLI::UpdateMasterCommand::Help => JLI::show_update_master_command_help(),
        JLI::UpdateMasterCommand::Update => {
            println!("Updating master password");
        }
    }
}
