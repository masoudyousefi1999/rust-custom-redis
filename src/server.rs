use std::{
    io::{Read, Write},
    net::TcpStream,
};

use crate::{
    error::AppResult,
    extractors,
    functions::{
        del_command, exists_command, expire_command, get_command, keys_command, set_command,
        ttl_command,
    },
    shared,
    structs::db::DataBase,
};

pub fn run_server(mut stream: &TcpStream, mut db: &mut DataBase) -> AppResult<()> {
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
                let response = set_command(&mut db, key, value);

                shared::write_response(stream, response)?;
            }
            "GET" => {
                let response = get_command(db, key);

                shared::write_response(stream, response)?;
            }
            "DEL" => {
                let response = del_command(db, key);

                shared::write_response(stream, response)?;
            }
            "EXISTS" => {
                let response = exists_command(db, key);
                shared::write_response(stream, response)?;
            }
            "KEYS" => {
                let response = keys_command(db);

                shared::write_response(stream, response)?;
            }
            "EXPIRE" => {
                let response = expire_command(&mut db, key, value);

                shared::write_response(stream, response)?;
            }
            "TTL" => {
                let response = ttl_command(db, key);

                shared::write_response(stream, response)?;
            }
            "PING" => {
                let response = extractors::resp_simple_string("PONG");
                stream.write_all(response.as_bytes())?;
            }

            "COMMANDS" => {
                let available_commands = extractors::resp_array(vec![
                    "set key value".to_string(),
                    "get key".to_string(),
                    "del key".to_string(),
                    "exists key".to_string(),
                    "expire key time(in sec)".to_string(),
                    "ttl key".to_string(),
                    "keys".to_string(),
                ]);

                stream.write_all(available_commands.as_bytes())?;
            }
            _ => {
                let response = extractors::resp_error("uknown commands.");
                stream.write_all(response.as_bytes())?;
            }
        };
    }

    Ok(())
}
