use std::{collections::HashMap, time::Instant};

use crate::structs::InsertInput;

#[derive(Debug)]
pub struct State {
    pub value: String,
    pub expire: Option<Instant>,
}

impl State {
    pub fn new(value: &str, expire: Option<Instant>) -> Self {
        Self {
            value: value.to_owned(),
            expire,
        }
    }
}

// impl ToOwned for State {
//     type Owned = Self;

//     fn clone_into(&self, target: &mut Self::Owned) {

//     }
//     fn to_owned(&self) -> Self::Owned {
//         self.into()
//     }

// }
pub struct DataBase(HashMap<String, State>);

impl DataBase {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

impl DataBase {
    pub fn insert(&mut self, input: InsertInput) {
        let state = State {
            value: input.state.value.to_owned(),
            expire: input.state.expire,
        };

        self.0.insert(input.key.to_owned(), state);
    }

    pub fn get(&self, k: &str) -> Option<&State> {
        self.0.get(k).and_then(|data| Some(data.to_owned()))
    }

    pub fn del(&mut self, k: &str) -> Option<State> {
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
