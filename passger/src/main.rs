mod commanders;
use commanders as Commanders;
use jli::core as JLI;

fn main() {
    JLI::show_welcome();
    let command = JLI::parse_commands();

    match command {
        JLI::Commands::Help => JLI::show_help(),
        JLI::Commands::Version => JLI::show_version(),
        JLI::Commands::Add(add_command) => Commanders::add_commander(add_command),
        _ => JLI::show_help(),
    }

    JLI::show_footer();
}
