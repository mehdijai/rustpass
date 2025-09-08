use jli::core as JLI;

use crate::{master_password::validate_master_password, pass_key::PassKey};

pub fn update_details_commander(command: JLI::UpdateDetailsCommand) {
    JLI::show_command_title("Update passkey details (Name, and/or email)");

    match command {
        JLI::UpdateDetailsCommand::Help => JLI::show_update_details_command_help(),
        JLI::UpdateDetailsCommand::Update { id, name, email } => {
            println!("Id: {}", id);
            println!("Name: {:?}", name);
            println!("Email: {:?}", email);
            let mut orm = db_manager::create_orm::<PassKey>();
            let found_passkey = orm.find_by_id(&id);

            if found_passkey.is_none() {
                println!("❌ Passkey not found");
                return;
            }

            if !validate_master_password() {
                eprintln!("Master password is not valid!");
                return;
            }

            let mut passkey = found_passkey.unwrap();

            passkey.update(name, email);

            let updated = orm.update(passkey);

            if updated {
                println!("✅ Passkey updated");
            } else {
                println!("❌ Passkey not updated");
            }
        }
    }
}
