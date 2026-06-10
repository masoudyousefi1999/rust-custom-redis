pub fn extractor(value: String) -> Vec<String> {
    let mut commands = value.lines().map(|v| v.to_owned()).collect::<Vec<String>>();

    commands.retain(|data| {
        if data.starts_with("*") | data.starts_with("$") {
            return false;
        }

        return true;
    });

    commands
}

pub fn resp_simple_string(value: &str) -> String {
    format!("+{value}\r\n")
}

pub fn resp_empty_data() -> String {
    "*-1\r\n".to_string()
}

pub fn resp_string(value: &str) -> String {
    let value_length = value.len();
    format!("${value_length}\r\n{value}\r\n")
}

pub fn resp_integer(value: i128) -> String {
    format!(":{value}\r\n")
}

pub fn resp_array(value: Vec<String>) -> String {
    let vec_length = value.len();
    let mut lines = String::new();

    for item in value {
        let item_length = item.len();
        let current_add = format!("${item_length}\r\n{item}\r\n");
        lines.push_str(&current_add);
    }

    format!("*{vec_length}\r\n{lines}")
}

pub fn resp_error(value: &str) -> String {
    format!("-ERR {value}\r\n")
}
