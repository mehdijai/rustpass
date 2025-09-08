use crate::{
    master_password::validate_master_password,
    pass_key::PassKey,
    utils::{create_id, read_secret},
};
use jli::core as JLI;

pub fn add_commander(command: JLI::AddCommand) {
    JLI::show_command_title("Register new passkey");

    match command {
        JLI::AddCommand::Help => JLI::show_add_command_help(),
        JLI::AddCommand::Add { name, email } => {
            println!("Name: {}", name);
            println!("Email: {}", email);
            println!("Please type the secret passkey:");

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

            let new_passkey = PassKey::create(&name, &email, &password, &id);
            orm.insert(&new_passkey);
            println!("Passkey registered successfully!");
            println!("{}", new_passkey);
        }
    }
}
