use jli::core as JLI;

use crate::{master_password::validate_master_password, pass_key::PassKey};

pub fn show_commander(command: JLI::ShowCommand) {
    JLI::show_command_title("Show a passkey");

    match command {
        JLI::ShowCommand::Help => JLI::show_show_command_help(),
        JLI::ShowCommand::Show { id } => {
            let orm = db_manager::create_orm::<PassKey>();
            let found_passkey = orm.find_by_id(&id);

            match found_passkey {
                Some(passkey) => {
                    println!("A match found");
                    if !validate_master_password() {
                        eprintln!("Master password is not valid!");
                        return;
                    }
                    println!("{}", passkey);
                    passkey.get_password();
                }
                None => {
                    println!("No passkey matching the id: \"{id}\"");
                }
            }
        }
    }
}
