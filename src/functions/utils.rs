use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::{AppError, AppResult};

pub fn get_current_mili_sec() -> AppResult<u128> {
    let current_system_time = SystemTime::now();
    let duration_since_epoch = current_system_time
        .duration_since(UNIX_EPOCH)
        .map_err(|_| AppError::ServerError("error on getting time duration".to_string()))?;
    let milliseconds_timestamp = duration_since_epoch.as_millis();

    Ok(milliseconds_timestamp)
}

pub fn set_expire_time(time: u128) -> AppResult<u128> {
    Ok(get_current_mili_sec()? + time * 1000)
}

pub fn expire_time(time: u128) -> AppResult<u128> {
    let current_time = get_current_mili_sec()?;

    if time > current_time {
        return Ok((time - current_time) / 1000);
    }

    return Ok(0);
}
