use jli::core as JLI;

use crate::{
    master_password::validate_master_password,
    pass_key::PassKey,
    utils::{create_id, read_secret, read_user_input},
};

pub fn create_commander(command: JLI::CreateCommand) {
    JLI::show_command_title("Create new passkey");

    match command {
        JLI::CreateCommand::Help => JLI::show_create_command_help(),
        JLI::CreateCommand::Create => {
            println!("Passkey creator wizard.");
            println!("Please enter the name of the passkey:");
            let name = read_user_input();
            println!("Please enter the username/email of the passkey:");
            let username = read_user_input();
            println!("Please enter the passkey:");
            let password = read_secret();

            if password.trim().len() == 0 {
                eprintln!("Password must be at least 1 character long!");
                return;
            }

            if !validate_master_password() {
                eprintln!("Master password is not valid!");
                return;
            }

            let mut orm = db_manager::create_orm::<PassKey>();
            let id = create_id(&orm, &name);

            let new_passkey = PassKey::create(&name, &username, &password, &id);

            orm.insert(&new_passkey);

            println!("Passkey created successfully");
            println!("{}", new_passkey);
        }
    }
}
