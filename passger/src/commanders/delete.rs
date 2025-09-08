use jli::core as JLI;

use crate::{master_password::validate_master_password, pass_key::PassKey};

pub fn delete_commander(command: JLI::DeleteCommand) {
    JLI::show_command_title("Delete a passkey");

    match command {
        JLI::DeleteCommand::Help => JLI::show_delete_command_help(),
        JLI::DeleteCommand::Delete { id } => {
            println!("Id: {}", id);
            if !validate_master_password() {
                eprintln!("Master password is not valid!");
                return;
            }
            let mut orm = db_manager::create_orm::<PassKey>();
            orm.delete(&id);
            println!("Passkey deleted successfully.");
        }
    }
}
