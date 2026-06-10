use crate::structs::db::State;

pub mod db;

pub struct DeleteInput {
    pub key: String,
}

impl DeleteInput {
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_owned(),
        }
    }
}

pub struct ExistsInput {
    pub key: String,
}

impl ExistsInput {
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_owned(),
        }
    }
}

pub struct InsertInput {
    pub key: String,
    pub state: State,
}

impl InsertInput {
    pub fn new(key: &str, state: State) -> Self {
        Self {
            key: key.to_owned(),
            state,
        }
    }
}

pub struct GetInput {
    pub key: String,
}

impl GetInput {
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_owned(),
        }
    }
}
