use crate::structs::{
    InsertInput,
    db::{DataBase, State},
};

use std::fs;

pub fn save(db: &DataBase) -> () {
    let data = db.get_all_data();
    let mut raw_data = String::new();

    for (item, state) in data {
        let data = format!(
            "{item}|{}|{}\n",
            state.value,
            state.expire.unwrap_or_else(|| 0)
        );

        raw_data.push_str(&data);
    }

    fs::write("./db.masoud", raw_data.as_bytes()).unwrap();
}

pub fn load(db: &mut DataBase) -> () {
    let file = fs::exists("./db.masoud").unwrap();

    if file {
        let opened_file = fs::read_to_string("./db.masoud").unwrap();

        let entry = opened_file.split("\n").collect::<Vec<&str>>();

        for item in entry {
            let data: Vec<&str> = item.split("|").collect();

            if data.len() == 3 {
                let key = data[0];
                let value = data[1];
                let expire: u128 = data[2].parse().unwrap();

                let expire = if expire > 0 { Some(expire) } else { None };

                let input = InsertInput::new(key, State::new(value, expire));
                db.insert(input);
            }
        }
    }
}
