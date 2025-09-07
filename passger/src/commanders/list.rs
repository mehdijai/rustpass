use crate::db as PassKeyDB;
use jli::core as JLI;

pub fn list_commander(command: JLI::ListCommand) {
    JLI::show_command_title("List of passkeys");

    match command {
        JLI::ListCommand::Help => JLI::show_list_command_help(),
        JLI::ListCommand::List => {
            let (orm, db) = PassKeyDB::init_db();
            let records_list = &orm.list();

            PassKeyDB::print_passkeys_table(&records_list);

            PassKeyDB::close_db((orm, db));
        }
    }
}
