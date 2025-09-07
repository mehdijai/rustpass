use std::{collections::HashMap, fs, path::PathBuf, process};

use crate::{get_db_path, handle_io_errors};

/// ## HasId Trait
/// Trait for ids of the ORM generic struct.
/// The struct must have both, and they are used in the ORM implementation
/// To filter and find unique records.
/// Repository::HasId
pub trait HasId {
    fn id(&self) -> &String;
    fn ulid(&self) -> &String;
}

/// ## Serializable trait
/// Handles the serializing/deserializing of a struct/object.
/// used in the initializing to parse the content string into records,
/// and deserialized to strings to save in DB.
pub trait Serializable {
    fn serialize(&self) -> String;
    fn deserialize(data: &str) -> Vec<Self>
    where
        Self: Sized;
}

/// ## Storage trait
/// Generic storage trait implemented in `DB` struct.
/// This Storage trait is for dependency injection,
/// as you can use the default `DB` Storage, which is
/// file based system. Or use a different storage system.
/// In tests, this Storage is used to mock the storage system.
pub trait Storage {
    fn read(&self) -> String;
    fn write(&self, data: &str);
}

/// ## ORM
/// ORM is the public API to manipulate data.
/// The data are extracted from the Storage system (DB),
/// deserialized, and pushed to the records HashMap,
/// the records are the DB records in-memory.
/// `changed` field is a private field switched to `true`,
/// when records is mutated. It's done manually in
/// the implementation of ORM methods. This field
/// is used to update the DB content is set to `true`,
/// hence we only write to DB if there is a change.
pub struct ORM<T: HasId + Clone + Serializable, S: Storage> {
    records: HashMap<String, T>,
    changed: bool,
    storage: S,
}

/// ### ORM Default implementation
/// The default implementation of the ORM struct.
/// It uses the `DB` struct as the Storage system.
/// The DB struct is a file based storage system.
impl<T: HasId + Clone + Serializable> ORM<T, DB> {
    pub fn new() -> Self {
        let storage = DB::new();
        let content = storage.read();

        let records = if content.trim().is_empty() {
            HashMap::new()
        } else {
            T::deserialize(&content)
                .into_iter()
                .map(|item| (item.ulid().to_string(), item))
                .collect()
        };

        Self {
            records,
            changed: false,
            storage,
        }
    }
}

/// ### ORM generic implementation
/// This implementation has `with_storage` method,
/// where you can use other `Storage`.
impl<T: HasId + Clone + Serializable, S: Storage> ORM<T, S> {
    pub fn with_storage(storage: S) -> Self {
        let content = storage.read();

        let records = if content.trim().is_empty() {
            HashMap::new()
        } else {
            T::deserialize(&content)
                .into_iter()
                .map(|item| (item.ulid().to_string(), item))
                .collect()
        };

        Self {
            records,
            changed: false,
            storage,
        }
    }

    pub fn find_by_ulid(&self, ulid: &str) -> Option<T> {
        self.records.get(ulid).cloned()
    }

    pub fn find_by_id(&self, id: &str) -> Option<T> {
        self.records.values().find(|item| item.id() == id).cloned()
    }

    pub fn list(&self) -> Vec<T> {
        self.records.values().cloned().collect()
    }

    pub fn insert(&mut self, payload: T) {
        self.changed = true;
        self.records.insert(payload.ulid().to_string(), payload);
    }

    pub fn update(&mut self, payload: T) -> bool {
        let exists = self.records.contains_key(payload.ulid());
        if !exists {
            return false;
        }

        self.changed = true;
        self.records.insert(payload.ulid().to_string(), payload);
        true
    }

    pub fn delete(&mut self, ulid: &str) -> bool {
        let exists = self.records.contains_key(ulid);
        if !exists {
            return false;
        }

        self.changed = true;
        self.records.remove(ulid);
        true
    }
}

/// ### ORM Drop implementation
/// At dropping, we check if there is a change,
/// If so, we write to the Storage. Otherwise, nothing happens,
/// and records are cleared from memory.
impl<T: HasId + Clone + Serializable, S: Storage> Drop for ORM<T, S> {
    fn drop(&mut self) {
        if self.changed {
            let serialized_data = self
                .list()
                .iter()
                .map(|item| item.serialize())
                .collect::<Vec<String>>()
                .join("\n");

            self.storage.write(&serialized_data);
        }
    }
}

/// ## DB
/// A storage system that uses a file.
pub struct DB {
    path: PathBuf,
    lock_path: PathBuf,
}

/// ### DB implementation
/// The DB file based system, reads the content of the DB file,
/// and locks it at the initialization.
/// If the file is locked, an error occurs
impl DB {
    fn new() -> Self {
        let path = get_db_path();
        let lock_path = path.with_extension("lock");

        if lock_path.exists() {
            com::print_error(com::Error::DBLocked);
        }

        let pid = process::id().to_string();
        let lock_result = fs::write(&lock_path, pid);
        handle_io_errors(&path, lock_result);

        DB { path, lock_path }
    }

    fn remove_lock_file(&self) {
        if self.lock_path.exists() {
            let result = fs::remove_file(&self.lock_path);
            handle_io_errors(&self.path, result);
        }
    }
}

impl Storage for DB {
    fn read(&self) -> String {
        let result = fs::read_to_string(&self.path);
        handle_io_errors(&self.path, result)
    }

    fn write(&self, data: &str) {
        let result = fs::write(&self.path, data);
        handle_io_errors(&self.path, result);
    }
}

impl Drop for DB {
    fn drop(&mut self) {
        self.remove_lock_file();
    }
}

pub fn create_orm<T: HasId + Clone + Serializable>() -> ORM<T, DB> {
    ORM::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[derive(Clone, Debug, PartialEq)]
    struct TestRecord {
        id: String,
        ulid: String,
        name: String,
    }

    impl HasId for TestRecord {
        fn id(&self) -> &String {
            &self.id
        }

        fn ulid(&self) -> &String {
            &self.ulid
        }
    }

    impl Serializable for TestRecord {
        fn serialize(&self) -> String {
            format!("{}|{}|{}", self.id, self.ulid, self.name)
        }

        fn deserialize(data: &str) -> Vec<Self> {
            data.lines()
                .filter(|line| !line.trim().is_empty())
                .map(|line| {
                    let parts: Vec<&str> = line.split('|').collect();
                    TestRecord {
                        id: parts[0].to_string(),
                        ulid: parts[1].to_string(),
                        name: parts[2].to_string(),
                    }
                })
                .collect()
        }
    }

    struct MockStorage {
        data: RefCell<String>,
        write_calls: RefCell<Vec<String>>,
    }

    impl MockStorage {
        fn new(initial_data: &str) -> Self {
            Self {
                data: RefCell::new(initial_data.to_string()),
                write_calls: RefCell::new(Vec::new()),
            }
        }
    }

    impl Storage for MockStorage {
        fn read(&self) -> String {
            self.data.borrow().clone()
        }

        fn write(&self, data: &str) {
            *self.data.borrow_mut() = data.to_string();
            self.write_calls.borrow_mut().push(data.to_string());
        }
    }

    #[test]
    fn test_new_orm_with_empty_storage() {
        let storage = MockStorage::new("");
        let orm: ORM<TestRecord, _> = ORM::with_storage(storage);

        assert_eq!(orm.list().len(), 0);
    }

    #[test]
    fn test_new_orm_with_existing_data() {
        let initial_data = "user1|ulid1|John\nuser2|ulid2|Jane";
        let storage = MockStorage::new(initial_data);
        let orm: ORM<TestRecord, _> = ORM::with_storage(storage);

        assert_eq!(orm.list().len(), 2);
        assert!(orm.find_by_id("user1").is_some());
        assert!(orm.find_by_ulid("ulid2").is_some());
    }

    #[test]
    fn test_insert() {
        let storage = MockStorage::new("");
        let mut orm: ORM<TestRecord, _> = ORM::with_storage(storage);

        let record = TestRecord {
            id: "user1".to_string(),
            ulid: "ulid1".to_string(),
            name: "John".to_string(),
        };

        orm.insert(record.clone());

        assert_eq!(orm.list().len(), 1);
        assert_eq!(orm.find_by_id("user1"), Some(record));
    }

    #[test]
    fn test_update_existing() {
        let initial_data = "user1|ulid1|John";
        let storage = MockStorage::new(initial_data);
        let mut orm: ORM<TestRecord, _> = ORM::with_storage(storage);

        let updated_record = TestRecord {
            id: "user1".to_string(),
            ulid: "ulid1".to_string(),
            name: "John Updated".to_string(),
        };

        let result = orm.update(updated_record.clone());

        assert!(result);
        assert_eq!(orm.find_by_ulid("ulid1"), Some(updated_record));
    }

    #[test]
    fn test_update_nonexistent() {
        let storage = MockStorage::new("");
        let mut orm: ORM<TestRecord, _> = ORM::with_storage(storage);

        let record = TestRecord {
            id: "user1".to_string(),
            ulid: "ulid1".to_string(),
            name: "John".to_string(),
        };

        let result = orm.update(record);

        assert!(!result);
        assert_eq!(orm.list().len(), 0);
    }

    #[test]
    fn test_delete_existing() {
        let initial_data = "user1|ulid1|John\nuser2|ulid2|Jane";
        let storage = MockStorage::new(initial_data);
        let mut orm: ORM<TestRecord, _> = ORM::with_storage(storage);

        let result = orm.delete("ulid1");

        assert!(result);
        assert_eq!(orm.list().len(), 1);
        assert!(orm.find_by_ulid("ulid1").is_none());
        assert!(orm.find_by_ulid("ulid2").is_some());
    }

    #[test]
    fn test_delete_nonexistent() {
        let storage = MockStorage::new("");
        let mut orm: ORM<TestRecord, _> = ORM::with_storage(storage);

        let result = orm.delete("nonexistent");

        assert!(!result);
    }

    #[test]
    fn test_find_by_id() {
        let initial_data = "user1|ulid1|John\nuser2|ulid2|Jane";
        let storage = MockStorage::new(initial_data);
        let orm: ORM<TestRecord, _> = ORM::with_storage(storage);

        let found = orm.find_by_id("user1");
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "John");

        let not_found = orm.find_by_id("user3");
        assert!(not_found.is_none());
    }

    #[test]
    fn test_find_by_ulid() {
        let initial_data = "user1|ulid1|John\nuser2|ulid2|Jane";
        let storage = MockStorage::new(initial_data);
        let orm: ORM<TestRecord, _> = ORM::with_storage(storage);

        let found = orm.find_by_ulid("ulid2");
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Jane");

        let not_found = orm.find_by_ulid("ulid3");
        assert!(not_found.is_none());
    }

    #[test]
    fn test_auto_save_on_drop() {
        let storage = MockStorage::new("");
        {
            let mut orm: ORM<TestRecord, _> = ORM::with_storage(storage);

            let record = TestRecord {
                id: "user1".to_string(),
                ulid: "ulid1".to_string(),
                name: "John".to_string(),
            };

            orm.insert(record);
        }
    }

    #[test]
    fn test_no_save_when_unchanged() {
        let storage = MockStorage::new("user1|ulid1|John");
        {
            let orm: ORM<TestRecord, _> = ORM::with_storage(storage);
            let _ = orm.list();
            let _ = orm.find_by_id("user1");
        }
    }
}
