use std::collections::HashMap;

pub struct DataBase(HashMap<String, String>);

impl DataBase {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

impl DataBase {
    pub fn insert(&mut self, k: &str, v: &str) {
        self.0.insert(k.to_owned(), v.to_owned());
    }

    pub fn get(&self, k: &str) -> Option<String> {
        self.0.get(k).and_then(|data| Some(data.to_owned()))
    }

    pub fn del(&mut self, k: &str) -> Option<String> {
        self.0.remove(k)
    }

    pub fn keys(&self) -> Vec<String> {
        self.0.keys().map(|key| key.to_owned()).collect()
    }

    pub fn exists(&self, k: &str) -> bool {
        let is_key_exist = self.0.get(k);

        if let None = is_key_exist {
            return false;
        }

        true
    }
}
