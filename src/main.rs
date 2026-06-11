use std::{io::Write, net::TcpListener};

use crate::{extractors::resp_error, server::run_server, structs::db::DataBase};

mod error;
mod extractors;
mod functions;
mod save_db;
mod server;
mod shared;
mod structs;

fn main() {
    let binding = TcpListener::bind("127.0.0.1:3030").expect("failed to listen TCP on port :3030");
    let mut db = DataBase::new();

    save_db::load(&mut db);

    for stream in binding.incoming() {
        println!("client connected");
        let mut stream = stream.expect("failed to parse stream");

        if let Err(err) = run_server(&stream, &mut db) {
            match err {
                error::AppError::ServerError(data) => {
                    stream
                        .write_all(resp_error(&data).as_bytes())
                        .expect("failed to send error!");
                }

                _ => unreachable!(),
            }
        }
    }
}
