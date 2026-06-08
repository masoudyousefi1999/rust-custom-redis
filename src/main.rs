use crate::server::run_server;

mod error;
mod functions;
mod server;
mod structs;
mod extractors;

fn main() {
    if let Err(err) = run_server() {
        eprintln!("error occured: {err:#?}");
    }
}
