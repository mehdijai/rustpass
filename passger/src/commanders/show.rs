use jli::core as JLI;

use crate::pass_key::PassKey;

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
                    println!("{}", passkey);
                }
                None => {
                    println!("No passkey matching the id: \"{id}\"");
                }
            }
        }
    }
}
