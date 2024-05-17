use serde_json::{self, Value};
use std::fs;
use std::io::{self, Read};
use thiserror::Error;

// Using custom errors as serde_json doesn't have "?" functionality.
#[derive(Error, Debug)]
pub enum JsonLoadError {
    // Defining the Io error, handy formatter from thiserror crate.
    #[error("Error while loading JSON file: {0}")]
    // Automatically implements From<io::Error> for JsonLoadError, converting Io err -> JsonLoadError
    Io(#[from] io::Error),
    #[error("Error parsing JSON (possibly invalid): {0}")]
    Parse(#[from] serde_json::Error),
}

// Function to load a JSON string and parse it into a serde_json JSON object.
pub fn load_json(file_path: &str) -> Result<Value, JsonLoadError> {
    let mut file = fs::File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    serde_json::from_str(&content).map_err(|e| JsonLoadError::Parse(e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helper::path::construct_file_path;

    #[test]
    fn test_read_broken() {
        let file_path = construct_file_path("src/data/test_broken.json");
        assert!(load_json(&file_path).is_err());
    }

    #[test]
    fn test_read_simple() {
        let file_path = construct_file_path("src/data/test_simple.json");
        let result = load_json(&file_path);
        assert!(result.is_ok());
    }
    #[test]
    fn test_read_nested() {
        let file_path = construct_file_path("src/data/test_nested.json");
        let result = load_json(&file_path);
        assert!(result.is_ok());
    }
    #[test]
    fn test_read_mixed_data() {
        let file_path = construct_file_path("src/data/test_mixed_data.json");
        let result = load_json(&file_path);
        assert!(result.is_ok());
    }
}
