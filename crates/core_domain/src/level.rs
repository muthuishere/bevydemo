// SRP: Level definition and state management
use serde::{Deserialize, Serialize};
use crate::math_engine::Operation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelConfig {
    pub id: u32,
    pub operation: Operation,
    pub max_operand_a: i32,
    pub max_operand_b: i32,
    pub puzzle_count: u32,
    pub mastery_threshold: f32,
    pub story_scene_id: String,
    pub reward_badge: String,
    pub title_key: String,
    pub description_key: String,
    pub voice_line_key: String,
    pub stars_to_unlock: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LevelStatus {
    Locked,
    Available,
    Completed { stars: u8 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelState {
    pub config_id: u32,
    pub status: LevelStatus,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_status_variants() {
        let locked = LevelStatus::Locked;
        let completed = LevelStatus::Completed { stars: 3 };
        assert_ne!(locked, completed);
        assert_eq!(locked, LevelStatus::Locked);
    }
}
