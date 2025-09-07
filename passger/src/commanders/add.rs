use jli::core as JLI;

use crate::pass_key::PassKey;

pub fn add_commander(command: JLI::AddCommand) {
    JLI::show_command_title("Register new passkey");

    match command {
        JLI::AddCommand::Help => JLI::show_add_command_help(),
        JLI::AddCommand::Add { name, email } => {
            println!("Name: {}", name);
            println!("Email: {}", email);
            println!("Please type the secret passkey:");
            let mut input = String::new();
            let input_result = std::io::stdin().read_line(&mut input);
            match input_result {
                Ok(_) => (),
                Err(err) => com::print_error(com::Error::InvalidData(err.to_string())),
            };
            let password = input.trim();
            let new_passkey = PassKey::create(&name, &email, password);
            let mut orm = db_manager::create_orm::<PassKey>();
            orm.insert(&new_passkey);
            println!("Passkey registered successfully!");
            println!("{}", new_passkey);
        }
    }
}
