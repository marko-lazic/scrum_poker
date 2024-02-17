use std::sync::Arc;

use crate::username;

pub const ALPHABET_AND_NUMBERS: [char; 62] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B',
    'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z',
];

pub fn username(username: &String) -> String {
    let filtered_name: String = username
        .trim()
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");
    let validated_name = if filtered_name.is_empty() {
        username::random_username()
    } else {
        filtered_name
    };

    return validated_name;
}

pub fn room_id(room_id: Arc<str>) -> Arc<str> {
    let filtered_chars: String = room_id
        .chars()
        .filter(|c| ALPHABET_AND_NUMBERS.contains(c))
        .take(10)
        .collect();

    return Arc::from(filtered_chars);
}
