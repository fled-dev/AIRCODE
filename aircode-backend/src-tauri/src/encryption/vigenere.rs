pub fn encrypt(message: &str, key: &str) -> String {
    if key.is_empty() || !key.chars().all(|c| c.is_ascii_alphabetic()) {
        // Return original message or handle error if key is invalid (empty or non-alphabetic)
        // For simplicity, returning original message here. A proper error might be better.
        return message.to_string();
    }

    let key_bytes = key.as_bytes();
    let mut key_iter = key_bytes.iter().cycle(); // Cycle through the key bytes

    message
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                // Get the next key character, ensuring it's uppercase for shift calculation
                let key_char = key_iter.next().unwrap().to_ascii_uppercase();
                let key_shift = key_char - b'A'; // Calculate shift amount (A=0, B=1, ...)

                let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                let shifted = base + ((c as u8 - base + key_shift) % 26);
                shifted as char
            } else {
                c // Keep non-alphabetic characters unchanged
            }
        })
        .collect()
}
