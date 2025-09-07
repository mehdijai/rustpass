use std::collections::HashMap;

pub trait HasId {
    fn id(&self) -> &String;
    fn ulid(&self) -> &String;
}
pub struct ORM<T: HasId + Clone> {
    records: HashMap<String, T>,
    pub changed: bool,
}

impl<T: HasId + Clone> ORM<T> {
    pub fn new(records: Vec<T>) -> Self {
        Self {
            records: records
                .into_iter()
                .map(|item| (item.ulid().to_string(), item))
                .collect(),
            changed: false,
        }
    }

    /// ## Find record by unique ULID
    pub fn find_by_ulid(&self, ulid: &str) -> Option<T> {
        self.records.get(ulid).cloned()
    }

    /// ## Find record by humain readable id
    pub fn find_by_id(&self, id: &str) -> Option<T> {
        if let Some(found_value) = self.records.iter().find(|item| item.1.id() == id) {
            found_value.1.clone().into()
        } else {
            None
        }
    }

    /// ## List all
    pub fn list(&self) -> Vec<T> {
        self.records.values().cloned().collect()
    }

    /// ## Insert new item
    pub fn insert(&mut self, payload: T) {
        self.changed = true;
        self.records.insert(payload.ulid().to_string(), payload);
    }

    /// ## Update an element
    pub fn update(&mut self, payload: T) -> bool {
        self.changed = true;
        let exists = self.records.contains_key(payload.ulid().as_str());

        if !exists {
            return false;
        }

        self.records.insert(payload.ulid().to_string(), payload);

        true
    }

    /// ## Delete an item
    pub fn delete(&mut self, ulid: &str) -> bool {
        self.changed = true;
        let exists = self.records.contains_key(ulid);

        if !exists {
            return false;
        }

        self.records.remove(ulid);

        true
    }
}
