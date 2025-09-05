use jli::core as JLI;

pub fn init_commander(command_args: Result<JLI::InitCommand, String>) {
    JLI::show_command_title("Initialize new vault");

    let command = JLI::handle_command_err(command_args);

    match command {
        JLI::InitCommand::Help => JLI::show_init_command_help(),
        JLI::InitCommand::Init => {
            println!("Initializing new vault...");
        }
    }
}
