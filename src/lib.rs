use std::collections::HashMap;

mod union_find;

#[derive(Debug)]
pub struct Ufdb {
    keys: HashMap<String, usize>,
    uf: union_find::UnionFind,
}

impl Ufdb {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
            uf: union_find::UnionFind::new(),
        }
    }

    pub fn make_set(&mut self, key: &str) -> bool {
        if !self.keys.contains_key(key) {
            let index = self.uf.add();

            self.keys.insert(key.to_string(), index);

            true
        } else {
            false
        }
    }

    pub fn unite(&mut self, key_a: &str, key_b: &str) -> bool {
        self.make_set(key_a);
        self.make_set(key_b);

        let a_index = self.keys[key_a];
        let b_index = self.keys[key_b];

        self.uf.unite(a_index, b_index)
    }

    pub fn same(&mut self, key_a: &str, key_b: &str) -> bool {
        let (Some(a_index), Some(b_index)) = (self.keys.get(key_a), self.keys.get(key_b)) else {
            return false;
        };

        self.uf.same(*a_index, *b_index)
    }

    pub fn groups(&mut self) -> HashMap<usize, Vec<&String>> {
        let mut groups = HashMap::new();

        for (key, index) in self.keys.iter() {
            let root = self.uf.find(*index);

            groups.entry(root).or_insert_with(Vec::new).push(key);
        }

        groups
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unite_auto_registers_unknown_keys_and_connects_them() {
        let mut ufdb = Ufdb::new();

        let merged = ufdb.unite("a", "b");

        assert!(merged);
        assert_eq!(ufdb.keys.len(), 2);

        let a_index = ufdb.keys["a"];
        let b_index = ufdb.keys["b"];

        assert!(ufdb.uf.same(a_index, b_index));
    }

    #[test]
    fn same_returns_true_after_unite() {
        let mut ufdb = Ufdb::new();

        ufdb.unite("a", "b");

        assert!(ufdb.same("a", "b"));
    }

    #[test]
    fn same_returns_false_when_not_united() {
        let mut ufdb = Ufdb::new();

        ufdb.make_set("a");
        ufdb.make_set("b");

        assert!(!ufdb.same("a", "b"));
    }

    #[test]
    fn same_returns_false_for_unregistered_keys_without_registering_them() {
        let mut ufdb = Ufdb::new();

        assert!(!ufdb.same("a", "b"));
        assert_eq!(ufdb.keys.len(), 0);
    }
}
