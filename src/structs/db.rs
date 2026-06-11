use std::collections::HashMap;

use crate::structs::InsertInput;

#[derive(Debug)]
pub struct State {
    pub value: String,
    pub expire: Option<u128>,
}

impl State {
    pub fn new(value: &str, expire: Option<u128>) -> Self {
        Self {
            value: value.to_owned(),
            expire,
        }
    }
}

pub struct DataBase(HashMap<String, State>);

impl DataBase {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

impl DataBase {
    pub fn insert(&mut self, input: InsertInput) {
        let state = State {
            value: input.state.value,
            expire: input.state.expire,
        };

        self.0.insert(input.key, state);
    }

    pub fn get(&self, k: &str) -> Option<&State> {
        self.0.get(k)
    }

    pub fn del(&mut self, k: &str) -> Option<State> {
        self.0.remove(k)
    }

    pub fn keys(&self) -> Vec<&str> {
        self.0.keys().map(String::as_str).collect()
    }

    pub fn exists(&self, k: &str) -> bool {
        let is_key_exist = self.0.get(k);

        if let None = is_key_exist {
            return false;
        }

        true
    }

    pub fn get_all_data(&self) -> &HashMap<String, State> {
        &self.0
    }

    pub fn insert_fake(&mut self) {
        self.0.insert(
            "test 1".to_string(),
            State {
                value: "test 1".to_owned(),
                expire: Some(10000),
            },
        );

        self.0.insert(
            "test 2".to_string(),
            State {
                value: "test 2".to_owned(),
                expire: Some(10000),
            },
        );

        self.0.insert(
            "test 3".to_string(),
            State {
                value: "test 3".to_owned(),
                expire: None,
            },
        );
    }

    pub fn insert_many(&mut self, items: HashMap<&str, State>) {
        for (key, state) in items {
            self.0.insert(key.to_owned(), state);
        }
    }
}
