// SRP: Maps logical asset names to file paths
use std::collections::HashMap;

pub struct AssetRegistry {
    paths: HashMap<String, String>,
}

impl AssetRegistry {
    pub fn new() -> Self {
        Self {
            paths: HashMap::new(),
        }
    }

    pub fn register(&mut self, key: &str, path: &str) {
        self.paths.insert(key.to_string(), path.to_string());
    }

    pub fn resolve(&self, key: &str) -> Option<&str> {
        self.paths.get(key).map(|s| s.as_str())
    }
}

impl Default for AssetRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_and_resolve() {
        let mut reg = AssetRegistry::new();
        reg.register("scene_forest_1", "scenes/forest_1.glb");
        assert_eq!(reg.resolve("scene_forest_1"), Some("scenes/forest_1.glb"));
        assert_eq!(reg.resolve("nonexistent"), None);
    }
}
