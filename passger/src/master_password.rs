use crate::utils::read_secret;

pub fn validate_master_password() -> bool {
    println!("Please enter your master password");
    let master_password = read_secret();
    master_password == "1234"
}
