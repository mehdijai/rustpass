use jli::core as JLI;

use crate::pass_key::{PassKey, print_passkeys_table};

pub fn list_commander(command: JLI::ListCommand) {
    JLI::show_command_title("List of passkeys");

    match command {
        JLI::ListCommand::Help => JLI::show_list_command_help(),
        JLI::ListCommand::List => {
            let orm = db_manager::create_orm::<PassKey>();
            let records_list = &orm.list();

            print_passkeys_table(&records_list);
        }
    }
}
