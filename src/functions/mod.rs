use crate::structs::{DeleteInput, ExistsInput, GetInput, InsertInput, db::DataBase};

pub fn del_from_db(db: &mut DataBase, input: DeleteInput) -> Option<String> {
    db.del(&input.key)
}

pub fn keys(db: &DataBase) -> Vec<String> {
    db.keys()
}

pub fn insert_to_db(db: &mut DataBase, input: &InsertInput) {
    db.insert(&input.key, &input.value);
}

pub fn get_from_db(db: &DataBase, input: GetInput) -> Option<String> {
    db.get(&input.key)
}

pub fn exists(db: &DataBase, input: ExistsInput) -> bool {
    db.exists(&input.key)
}
