// SRP: save/load player profile to disk as JSON
use core_domain::progression::PlayerProfile;
use std::fs;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PersistError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

pub struct ProfileStore {
    pub path: String,
}

impl ProfileStore {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }

    pub fn save(&self, profile: &PlayerProfile) -> Result<(), PersistError> {
        let json = serde_json::to_string_pretty(profile)?;
        fs::write(&self.path, json)?;
        Ok(())
    }

    pub fn load(&self) -> Result<PlayerProfile, PersistError> {
        let content = fs::read_to_string(&self.path)?;
        Ok(serde_json::from_str(&content)?)
    }

    pub fn load_or_default(&self, default_language: &str) -> PlayerProfile {
        self.load()
            .unwrap_or_else(|_| PlayerProfile::new(default_language))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_and_load_profile() {
        let path = "test_profile_persistence.json";
        let store = ProfileStore::new(path);
        let profile = PlayerProfile::new("es");
        store.save(&profile).unwrap();
        let loaded = store.load().unwrap();
        assert_eq!(loaded.preferred_language, "es");
        fs::remove_file(path).ok();
    }

    #[test]
    fn test_load_or_default_when_missing() {
        let store = ProfileStore::new("nonexistent_profile_xyz_abc.json");
        let profile = store.load_or_default("fr");
        assert_eq!(profile.preferred_language, "fr");
    }
}
