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
    pub value: String,
}

impl InsertInput {
    pub fn new(key: &str, value: &str) -> Self {
        Self {
            key: key.to_owned(),
            value: value.to_owned(),
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
