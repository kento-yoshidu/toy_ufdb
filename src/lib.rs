use std::collections::HashMap;

mod union_find;

#[derive(Debug)]
pub struct Ufdb {
    keys: HashMap<String, usize>,
    uf: union_find::UnionFind,
}

impl Ufdb {
    pub fn make_set(&mut self, key: &str) -> bool {
        if !self.keys.contains_key(key) {
            let index = self.uf.add();

            self.keys.insert(key.to_string(), index);

            true
        } else {
            false
        }
    }
}
