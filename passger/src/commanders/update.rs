use jli::core as JLI;

use crate::{
    pass_key::PassKey,
    utils::{read_user_input, yn_user_input},
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
                        println!("Please enter your master password");
                        let master_password = read_user_input();
                        if master_password != "1234" {
                            eprintln!("Wrong master password");
                            return;
                        }
                        println!("Please enter your new updated passkey");
                        let new_password = read_user_input();
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
