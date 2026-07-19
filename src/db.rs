use std::collections::HashMap;

use crate::Ufdb;

#[derive(Debug)]
pub struct Db {
    current_db: String,
    db: HashMap<String, Ufdb>,
}

impl Db {
    pub fn new() -> Self {
        Self {
            current_db: "ufdb".to_string(),
            db: HashMap::from([("ufdb".to_string(), Ufdb::new())]),
        }
    }

    pub fn current(&mut self) -> &mut Ufdb {
        self.db.get_mut(&self.current_db).unwrap()
    }

    pub fn create_db(&mut self, name: &str) -> bool {
        if !self.db.contains_key(name) {
            self.db.insert(name.to_string(), Ufdb::new());
            self.current_db = name.to_string();
            true
        } else {
            self.current_db = name.to_string();
            false
        }
    }

    pub fn use_db(&mut self, name: &str) -> bool {
        if self.db.contains_key(name) {
            self.current_db = name.to_string();
            true
        } else {
            false
        }
    }
}
