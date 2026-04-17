// SRP: loads and parses level configs from JSON
use core_domain::level::LevelConfig;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),
}

pub struct LevelLoader;

impl LevelLoader {
    pub fn load_from_str(json: &str) -> Result<Vec<LevelConfig>, LoadError> {
        Ok(serde_json::from_str(json)?)
    }

    pub fn load_from_file(path: &str) -> Result<Vec<LevelConfig>, LoadError> {
        let content = std::fs::read_to_string(path)?;
        Self::load_from_str(&content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_level_config_from_json() {
        let json = r#"[{
            "id": 1,
            "operation": "Addition",
            "max_operand_a": 5,
            "max_operand_b": 5,
            "puzzle_count": 5,
            "mastery_threshold": 0.8,
            "story_scene_id": "scene_forest_1",
            "reward_badge": "badge_star_1",
            "title_key": "level.1.title",
            "description_key": "level.1.description",
            "voice_line_key": "level.1.voice",
            "stars_to_unlock": 0
        }]"#;
        let levels = LevelLoader::load_from_str(json).unwrap();
        assert_eq!(levels.len(), 1);
        assert_eq!(levels[0].id, 1);
        assert_eq!(levels[0].max_operand_a, 5);
    }
}
