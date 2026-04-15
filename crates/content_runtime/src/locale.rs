// SRP: Loads and resolves localized strings
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LocaleError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Key not found: {0}")]
    KeyNotFound(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocaleData {
    pub language: String,
    pub strings: HashMap<String, String>,
}

impl LocaleData {
    pub fn from_str(json: &str) -> Result<Self, LocaleError> {
        Ok(serde_json::from_str(json)?)
    }

    pub fn get(&self, key: &str) -> Result<&str, LocaleError> {
        self.strings
            .get(key)
            .map(|s| s.as_str())
            .ok_or_else(|| LocaleError::KeyNotFound(key.to_string()))
    }

    pub fn get_or_key<'a>(&'a self, key: &'a str) -> &'a str {
        self.strings.get(key).map(|s| s.as_str()).unwrap_or(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locale_lookup() {
        let json = r#"{"language":"en","strings":{"level.1.title":"The Forest Adventure"}}"#;
        let locale = LocaleData::from_str(json).unwrap();
        assert_eq!(locale.get("level.1.title").unwrap(), "The Forest Adventure");
    }

    #[test]
    fn test_locale_missing_key_returns_key() {
        let json = r#"{"language":"en","strings":{}}"#;
        let locale = LocaleData::from_str(json).unwrap();
        assert_eq!(locale.get_or_key("missing.key"), "missing.key");
    }
}
