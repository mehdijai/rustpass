mod commanders;

use commanders as Commanders;
use jli::core as JLI;

fn main() {
    JLI::show_welcome();
    let command = JLI::parse_commands();

    match command {
        JLI::Commands::Help => JLI::show_help(),
        JLI::Commands::Version => JLI::show_version(),
        JLI::Commands::Add(command) => Commanders::add_commander(command),
        JLI::Commands::Init(command) => Commanders::init_commander(command),
        JLI::Commands::List(command) => Commanders::list_commander(command),
        _ => JLI::show_help(),
    }

    JLI::show_footer();
}
