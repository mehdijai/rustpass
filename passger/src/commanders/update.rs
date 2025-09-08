use jli::core as JLI;

use crate::{
    master_password::validate_master_password,
    pass_key::PassKey,
    utils::{read_secret, yn_user_input},
};

pub fn update_commander(command: JLI::UpdateCommand) {
    JLI::show_command_title("Update a passkey");

    match command {
        JLI::UpdateCommand::Help => JLI::show_update_command_help(),
        JLI::UpdateCommand::Update { id } => {
            println!("Id: {}", id);
            let mut orm = db_manager::create_orm::<PassKey>();
            let found_passkey = orm.find_by_id(&id);

            match found_passkey {
                Some(mut passkey) => {
                    println!("You want to update (Y/N):");
                    println!("{}", passkey);

                    if yn_user_input() {
                        if !validate_master_password() {
                            eprintln!("Master password is not valid!");
                            return;
                        }
                        println!("Please enter your new updated passkey");
                        let new_password = read_secret();
                        passkey.update_password(&new_password);
                        orm.update(passkey);
                        println!("Passkey updated successfully");
                    }
                }
                None => {
                    println!("Passkey not found");
                }
            }
        }
    }
}
