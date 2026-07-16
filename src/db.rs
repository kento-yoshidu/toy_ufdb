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
            db: HashMap::from("ufdb".to_string(), Ufdb::new()),
        },
    }

    pub fn current(&mut self) -> &mut Ufdb {
        self.db.get_mut(&self.current_db).unwrap()
    }
}