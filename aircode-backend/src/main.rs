use std::path::Path;
use std::process;

mod encryption; // Make the encryption module available

fn main() {
    // Define the path to the profile
    let profile_path = Path::new("profiles/vigenere.json"); // New path for Vigenere

    // Load the encryption profile
    let profile = match encryption::EncryptionProfile::load_from_json(profile_path) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error loading profile '{}': {}", profile_path.display(), e);
            process::exit(1);
        }
    };

    println!("Loaded profile: {:?}", profile);

    // Define the message and key for testing
    let message = "ATTACKATDAWN"; // New message for Vigenere test
    let key = &profile.example_key; // Use the example key from the profile

    println!("Original message: {}", message);
    println!("Using key: {}", key);

    // Encrypt the message using the loaded profile
    match profile.encrypt(message, key) {
        Ok(encrypted_message) => {
            println!("Encrypted message: {}", encrypted_message);
        }
        Err(e) => {
            eprintln!("Error encrypting message: {}", e);
            process::exit(1);
        }
    }
}
