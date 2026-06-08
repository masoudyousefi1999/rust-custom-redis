use std::{
    io::{Read, Write},
    net::TcpListener,
};

use crate::{
    error::AppResult,
    extractors::{
        self, resp_array, resp_empty_data, resp_integer, resp_simple_string, resp_string,
    },
    functions::{del_from_db, exists, get_from_db, insert_to_db, keys},
    structs::{DeleteInput, ExistsInput, GetInput, InsertInput, db::DataBase},
};

pub fn run_server() -> AppResult<()> {
    let binding = TcpListener::bind("127.0.0.1:3030")?;
    let mut db = DataBase::new();

    for stream in binding.incoming() {
        let mut stream = stream?;
        println!("client connected");

        // write_to_terminal(&stream, "connected successfully.", Colors::GREEN)?;
        // stream.write_all(b"+PONG\r\n")?;
        // stream.flush()?;

        loop {
            let mut buffer = [0; 1024];
            let bytes_read = stream.read(&mut buffer)?;

            if bytes_read == 0 {
                break;
            }

            let user_input = String::from_utf8_lossy(&buffer[..bytes_read]).into_owned();

            let commnads = extractors::extractor(user_input);
            let command = commnads[0].to_uppercase();

            let key = if commnads.len() >= 2 {
                Some(commnads[1].to_owned())
            } else {
                None
            };

            let value = if commnads.len() >= 3 {
                Some(commnads[2].to_owned())
            } else {
                None
            };

            match command.as_str() {
                "SET" => {
                    let input = InsertInput::new(&key.unwrap(), &value.unwrap());
                    insert_to_db(&mut db, &input);
                    let response = resp_simple_string("OK");
                    stream.write_all(response.as_bytes())?;
                }
                "GET" => {
                    let input = GetInput::new(&key.unwrap());
                    let extacted = get_from_db(&db, input);

                    if let Some(data) = extacted {
                        stream.write_all(resp_string(&data).as_bytes())?;
                    } else {
                        stream.write_all(resp_empty_data().as_bytes())?;
                    }
                }
                "DEL" => {
                    let input = DeleteInput::new(&key.unwrap());
                    let extracted = del_from_db(&mut db, input);

                    match extracted {
                        Some(_) => stream.write_all(resp_integer(1).as_bytes())?,
                        None => stream.write_all(resp_integer(0).as_bytes())?,
                    }
                }
                "EXISTS" => {
                    let input = ExistsInput::new(&key.unwrap());
                    let is_existed = exists(&db, input);

                    match is_existed {
                        true => stream.write_all(resp_integer(1).as_bytes())?,
                        false => stream.write_all(resp_integer(0).as_bytes())?,
                    }
                }
                "KEYS" => {
                    let keys = keys(&db);

                    if keys.len() > 0 {
                        stream.write_all(resp_array(keys).as_bytes())?;
                    } else {
                        stream.write_all(resp_empty_data().as_bytes())?;
                    }
                }
                "PING" => {
                    let response = extractors::resp_simple_string("PONG");
                    stream.write_all(response.as_bytes())?;
                }
                data => {
                    let response = extractors::resp_simple_string(data);
                    stream.write_all(response.as_bytes())?;
                }
            };
        }
    }

    Ok(())
}
