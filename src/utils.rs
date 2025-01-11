pub fn str_vec_non_empty(vec: &Vec<String>) -> bool {
    vec.iter().any(|s| !s.is_empty())
}

pub fn u8_to_str(u8: &Vec<u8>) -> String {
    match String::from_utf8(u8.clone()) {
        Ok(str) => str,
        Err(_) => String::from_utf8_lossy(&u8).to_string(),
    }
}

pub fn u8_to_str_vec(u8: Vec<u8>) -> Vec<String> {
    u8_to_str(&u8).lines().map(|s| s.to_string()).collect()
}
