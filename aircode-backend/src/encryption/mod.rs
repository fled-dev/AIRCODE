use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::error::Error;

pub mod caesar; // Declare caesar module
pub mod vigenere; // Declare vigenere module (empty for now)

#[derive(Serialize, Deserialize, Debug)]
pub struct EncryptionProfile {
    pub name: String,
    #[serde(rename = "type")] // Map JSON "type" to Rust field "profile_type"
    pub profile_type: String,
    pub key_required: bool,
    pub key_type: String, // "int" or "string"
    pub description: String,
    pub example_key: String,
}

impl EncryptionProfile {
    // Loads an encryption profile from a JSON file
    pub fn load_from_json(file_path: &Path) -> Result<Self, Box<dyn Error>> {
        let json_str = fs::read_to_string(file_path)?;
        let profile: EncryptionProfile = serde_json::from_str(&json_str)?;
        Ok(profile)
    }

    // Encrypts a message based on the profile type
    pub fn encrypt(&self, message: &str, key: &str) -> Result<String, Box<dyn Error>> {
        match self.profile_type.as_str() {
            "caesar" => {
                if self.key_type != "int" {
                    return Err("Caesar cipher requires an integer key.".into());
                }
                // Attempt to parse the key as an integer
                match key.parse::<i32>() {
                    Ok(int_key) => Ok(caesar::encrypt(message, int_key)),
                    Err(_) => Err(format!("Invalid integer key provided for Caesar: {}", key).into()),
                }
            }
            "vigenere" => {
                if self.key_type != "string" {
                    return Err("Vigenere cipher requires a string key.".into());
                }
                // Key validation (non-empty, alphabetic) happens inside vigenere::encrypt
                Ok(vigenere::encrypt(message, key))
            }
            _ => Err(format!("Unsupported encryption type: {}", self.profile_type).into()),
        }
    }
}
