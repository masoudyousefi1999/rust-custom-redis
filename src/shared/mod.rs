use std::{io::Write, net::TcpStream};

use crate::error::{AppError, AppResult};

pub fn write_response(mut stream: &TcpStream, response: AppResult<String>) -> AppResult<()> {
    match response {
        Ok(data) => stream.write_all(data.as_bytes())?,
        Err(AppError::LogicalError(err)) => stream.write_all(err.as_bytes())?,
        Err(err) => return Err(err),
    }

    Ok(())
}
