pub fn encrypt(message: &str, key: i32) -> String {
    message.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
            // Ensure key is positive for modulo operation
            let effective_key = key.rem_euclid(26);
            let shifted = base + ((c as u8 - base + effective_key as u8) % 26);
            shifted as char
        } else {
            c // Keep non-alphabetic characters unchanged
        }
    }).collect()
}