use core::fmt;
use std::borrow::Cow;

use db_manager as Repository;
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

impl PassKey {
    pub fn create(name: &str, username: &str, password: &str, id: String) -> Self {
        Self {
            id,
            ulid: Ulid::new().to_string(),
            name: name.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        }
    }

    pub fn update(&mut self, name: Option<String>, username: Option<String>) {
        match name {
            Some(name) => self.name = name,
            None => {}
        };
        match username {
            Some(username) => self.username = username,
            None => {}
        };
    }
}

impl Repository::HasId for PassKey {
    fn id(&self) -> &String {
        &self.id
    }
    fn ulid(&self) -> &String {
        &self.ulid
    }
}

impl Repository::Serializable for PassKey {
    fn serialize(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}",
            self.id, self.ulid, self.name, self.username, self.password
        )
    }

    fn deserialize(data: &str) -> Vec<Self> {
        data.lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                let parts: Vec<&str> = line.split('|').collect();
                PassKey {
                    id: parts[0].to_string(),
                    ulid: parts[1].to_string(),
                    name: parts[2].to_string(),
                    username: parts[3].to_string(),
                    password: parts[4].to_string(),
                }
            })
            .collect()
    }
}

// Formatting and display

impl Tabled for PassKey {
    const LENGTH: usize = 4;

    fn fields(&self) -> Vec<Cow<'_, str>> {
        vec![
            Cow::Borrowed(self.id.as_str()),
            Cow::Borrowed(self.name.as_str()),
            Cow::Borrowed(self.username.as_str()),
            Cow::Borrowed("***"),
        ]
    }

    fn headers() -> Vec<Cow<'static, str>> {
        vec![
            Cow::Borrowed("ID"),
            Cow::Borrowed("Site"),
            Cow::Borrowed("Username"),
            Cow::Borrowed("Password"),
        ]
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

pub fn print_passkeys_table(pass_keys: &[PassKey]) {
    let table = Table::new(pass_keys);
    println!("{}", table);
}
