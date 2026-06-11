use crate::{
    error::{AppError, AppResult},
    extractors,
    functions::utils::set_expire_time,
    structs::{
        DeleteInput, ExistsInput, GetInput, InsertInput,
        db::{DataBase, State},
    },
};

mod utils;

pub fn del_command(db: &mut DataBase, key: Option<String>) -> AppResult<String> {
    let key = key.ok_or(AppError::LogicalError(
        "key is required for del command.".to_string(),
    ))?;

    let input = DeleteInput::new(&key);
    let extracted = db.del(&input.key);

    if let None = extracted {
        return Ok(extractors::resp_integer(0));
    }

    return Ok(extractors::resp_integer(1));
}

pub fn keys_command(db: &DataBase) -> AppResult<String> {
    let keys = db.keys();

    if keys.len() == 0 {
        return Ok(extractors::resp_empty_data());
    }

    Ok(extractors::resp_array(&keys))
}

pub fn set_command(
    db: &mut DataBase,
    key: Option<String>,
    value: Option<String>,
) -> AppResult<String> {
    let key = key.ok_or(AppError::LogicalError(
        "key and value are required for set command".to_string(),
    ))?;

    let value = value.ok_or(AppError::LogicalError(
        "value is required for set command".to_string(),
    ))?;

    let input = InsertInput::new(&key, State::new(&value, None));

    db.insert(input);

    let response = extractors::resp_simple_string("OK");

    Ok(response)
}

pub fn get_command(db: &mut DataBase, key: Option<String>) -> AppResult<String> {
    let key = key.ok_or(AppError::LogicalError(
        "key is required for get command.".to_string(),
    ))?;

    let input = GetInput::new(&key);
    let extacted = db.get(&input.key);

    if let Some(data) = extacted {
        if let Some(expire) = data.expire {
            let expire_time = utils::expire_time(expire)?;

            if expire_time == 0 {
                db.del(&key);
                return Ok(extractors::resp_empty_data());
            }
        }
        return Ok(extractors::resp_string(&data.value));
    }

    return Ok(extractors::resp_empty_data());
}

pub fn expire_command(
    db: &mut DataBase,
    key: Option<String>,
    expire: Option<String>,
) -> AppResult<String> {
    let key = key.ok_or(AppError::LogicalError(
        "key and value are required for set command".to_string(),
    ))?;

    let expire: u128 = expire
        .ok_or(AppError::LogicalError(
            "time is required for expire command".to_string(),
        ))?
        .parse::<u128>()
        .map_err(|_| {
            AppError::LogicalError("expire must be number and must be posetive number".to_string())
        })?;

    let value = db.get(&key);

    let Some(current_value) = value else {
        return Err(AppError::LogicalError("key not founded.".to_string()));
    };

    let expire_time = set_expire_time(expire)?;

    let insert_input = InsertInput::new(&key, State::new(&current_value.value, Some(expire_time)));

    db.insert(insert_input);

    Ok(extractors::resp_integer(1))
}

pub fn exists_command(db: &DataBase, key: Option<String>) -> AppResult<String> {
    let input = ExistsInput::new(&key.ok_or(AppError::LogicalError(
        "key and value are required for set command".to_string(),
    ))?);

    let is_existed = db.exists(&input.key);

    match is_existed {
        true => return Ok(extractors::resp_integer(1)),
        false => return Ok(extractors::resp_integer(0)),
    };
}

pub fn ttl_command(db: &mut DataBase, key: Option<String>) -> AppResult<String> {
    let key = key.ok_or(AppError::LogicalError(
        "key is required for get command.".to_string(),
    ))?;

    let input = GetInput::new(&key);
    let extacted = db.get(&input.key);

    if let Some(data) = extacted {
        let Some(expire) = data.expire else {
            return Ok(extractors::resp_integer(-1));
        };

        let expire_time = utils::expire_time(expire)?;

        if expire_time == 0 {
            db.del(&key);
            return Ok(extractors::resp_integer(-2));
        }

        return Ok(extractors::resp_integer(expire_time as i128));
    }

    return Ok(extractors::resp_integer(-2));
}
