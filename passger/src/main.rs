mod commanders;
mod pass_key;
mod utils;

use commanders as Commanders;
use jli::core as JLI;

const TEST_MODE: bool = false;

fn main() {
    run();
}

fn test_mode() {}

fn run() {
    if TEST_MODE {
        test_mode();
        return;
    }

    JLI::show_welcome();
    let command = JLI::parse_commands();

    match command {
        JLI::Commands::Help => JLI::show_help(),
        JLI::Commands::Version => JLI::show_version(),
        JLI::Commands::Add(command) => Commanders::add_commander(command),
        JLI::Commands::Init(command) => Commanders::init_commander(command),
        JLI::Commands::List(command) => Commanders::list_commander(command),
        JLI::Commands::Create(command) => Commanders::create_commander(command),
        JLI::Commands::UpdateMaster(command) => Commanders::update_master_commander(command),
        JLI::Commands::Update(command) => Commanders::update_commander(command),
        JLI::Commands::Show(command) => Commanders::show_commander(command),
        JLI::Commands::Delete(command) => Commanders::delete_commander(command),
        JLI::Commands::UpdateDetails(command) => Commanders::update_details_commander(command),
    }

    JLI::show_footer();
}
