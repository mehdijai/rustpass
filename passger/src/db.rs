use core::fmt;
use db_manager as DBManager;
use tabled::{Table, Tabled};
use ulid::Ulid;

#[derive(Clone)]
pub struct PassKey {
    id: String,
    ulid: String,
    name: String,
    username: String,
    password: String,
}

fn generate_id(input: &str) -> String {
    input
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphabetic() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("-")
}

impl PassKey {
    fn new(string_value: &str) -> Self {
        Self::from_string(string_value)
    }

    pub fn create(name: &str, username: &str, password: &str) -> Self {
        Self {
            id: generate_id(&name),
            ulid: Ulid::new().to_string(),
            name: name.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        }
    }

    pub fn serialize(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}",
            self.id, self.ulid, self.name, self.username, self.password
        )
    }

    fn from_string(s: &str) -> Self {
        let parts: Vec<&str> = s.split('|').collect();
        if parts.len() != 5 {
            com::print_error(com::Error::CorruptedData);
        }
        Self {
            id: parts[0].to_string(),
            ulid: parts[1].to_string(),
            name: parts[2].to_string(),
            username: parts[3].to_string(),
            password: parts[4].to_string(),
        }
    }
}

impl fmt::Display for PassKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PassKey(id: {}, name: {}, username: {})",
            self.id, self.name, self.username
        )
    }
}

impl DBManager::HasId for PassKey {
    fn id(&self) -> &String {
        &self.id
    }
    fn ulid(&self) -> &String {
        &self.ulid
    }
}

#[derive(Debug, Clone, Tabled)]
pub struct PassKeyDisplay {
    #[tabled(rename = "ID")]
    pub id: String,
    #[tabled(rename = "Site")]
    pub name: String,
    #[tabled(rename = "Username")]
    pub username: String,
    #[tabled(rename = "Password")]
    pub password: String,
}

impl From<&PassKey> for PassKeyDisplay {
    fn from(pk: &PassKey) -> Self {
        Self {
            id: pk.id.clone(),
            name: pk.name.clone(),
            username: pk.username.clone(),
            password: "***".to_string(), // Hide password
        }
    }
}

pub fn print_passkeys_table(pass_keys: &[PassKey]) {
    let display_keys: Vec<PassKeyDisplay> = pass_keys.iter().map(|pk| pk.into()).collect();

    let table = Table::new(display_keys);
    println!("{}", table);
}

pub fn init_db() -> (DBManager::ORM<PassKey>, DBManager::DB) {
    let mut db = DBManager::DB::new();
    let init_content = db.get();

    let pass_keys = parse_lines(init_content.as_str());

    let orm = DBManager::ORM::new(pass_keys);

    db.close();

    (orm, db)
}

fn parse_lines(content: &str) -> Vec<PassKey> {
    let records: Vec<&str> = content.lines().collect();
    records.iter().map(|line| PassKey::new(line)).collect()
}

pub fn close_db((orm, mut db): (db_manager::ORM<PassKey>, db_manager::DB)) {
    if orm.changed {
        let list_items: Vec<String> = orm
            .list()
            .iter()
            .map(|pass_key| pass_key.serialize())
            .collect();
        let content: String = list_items.join("\n");
        db.write(content);
    }
    db.close();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_lines() {
        let content = "1|01G7Z1W1YX9Z7YXZ1YXZ1YXZ|example.com|user|pass
        2|01G7Z1W1YX9Z7YXZ1YXZ1YXZ|example.com|user|pass";

        let pass_keys = parse_lines(content);

        assert_eq!(pass_keys.len(), 2);

        for pass_key in pass_keys {
            assert_eq!(pass_key.ulid, "01G7Z1W1YX9Z7YXZ1YXZ1YXZ");
            assert_eq!(pass_key.name, "example.com");
            assert_eq!(pass_key.username, "user");
            assert_eq!(pass_key.password, "pass");
        }
    }
}
