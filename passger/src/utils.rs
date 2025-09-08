use crate::pass_key::PassKey;
use db_manager::HasId;
use jli::core as JLI;

pub fn create_id(orm: &db_manager::ORM<PassKey, db_manager::DB>, name: &str) -> String {
    let passkeys = orm.list();

    let normalized_name = JLI::normalize_string(&name);

    // TODO: Use Regex
    let id_duplicates = passkeys
        .iter()
        .filter(|passkey| {
            passkey.id() == &normalized_name
                || passkey
                    .id()
                    .starts_with(format!("{}-", &normalized_name).as_str())
        })
        .count();

    if id_duplicates == 0 {
        normalized_name
    } else {
        format!("{}-{}", normalized_name, id_duplicates + 1)
    }
}

pub fn yn_user_input() -> bool {
    let input = read_user_input();
    input.to_lowercase() == "y"
}

pub fn read_user_input() -> String {
    let mut input = String::new();
    let input_result = std::io::stdin().read_line(&mut input);
    match input_result {
        Ok(_) => (),
        Err(err) => com::print_error(com::Error::InvalidData(err.to_string())),
    };
    input.trim().to_string()
}

pub fn read_secret() -> String {
    let password_input = rpassword::read_password();
    match password_input {
        Ok(pass) => pass,
        Err(err) => com::print_error(com::Error::InvalidData(err.to_string())),
    }
}
