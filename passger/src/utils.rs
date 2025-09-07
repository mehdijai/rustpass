use crate::pass_key::PassKey;
use db_manager::HasId;
use jli::core as JLI;

pub fn create_id(orm: &db_manager::ORM<PassKey, db_manager::DB>, name: &str) -> String {
    let passkeys = orm.list();

    let normalized_name = JLI::normalize_string(&name);

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
