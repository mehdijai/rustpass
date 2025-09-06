use jli::core as JLI;

pub fn init_commander(command: JLI::InitCommand) {
    JLI::show_command_title("Initialize new vault");

    match command {
        JLI::InitCommand::Help => JLI::show_init_command_help(),
        JLI::InitCommand::Init => {
            println!("Initializing new vault...");
        }
    }
}
