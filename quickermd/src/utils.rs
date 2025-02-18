pub fn u8_to_str(u8: &Vec<u8>) -> String {
    match String::from_utf8(u8.clone()) {
        Ok(str) => str,
        Err(_) => String::from_utf8_lossy(&u8).to_string(),
    }
}
