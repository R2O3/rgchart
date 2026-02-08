pub fn get_keycount_from_str(str: &str) -> Option<u8> {
    match str.to_lowercase().strip_prefix("keys") {
        Some(num_str) => num_str.parse::<u8>().ok(),
        None => None
    }
}

pub fn get_mode_from_u8(keycount: u8) -> String {
    "keys".to_string() + &keycount.to_string()
}
