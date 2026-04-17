// SRP: Manages player profile, unlocks, overall progression
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::level::LevelStatus;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlayerProfile {
    pub version: u32,
    pub total_stars: u32,
    pub current_streak_days: u32,
    pub preferred_language: String,
    pub tts_enabled: bool,
    pub level_states: HashMap<u32, LevelStatus>,
}

impl PlayerProfile {
    pub fn new(default_language: &str) -> Self {
        let mut p = Self {
            version: 1,
            preferred_language: default_language.to_string(),
            tts_enabled: true,
            ..Default::default()
        };
        p.level_states.insert(1, LevelStatus::Available);
        p.level_states.insert(6, LevelStatus::Available);
        p
    }

    pub fn complete_level(&mut self, level_id: u32, stars: u8, stars_to_unlock_next: u32) {
        let prev_stars = match self.level_states.get(&level_id) {
            Some(LevelStatus::Completed { stars: s }) => *s,
            _ => 0,
        };
        let new_stars = stars.max(prev_stars);
        let gained = new_stars.saturating_sub(prev_stars) as u32;
        self.total_stars += gained;
        self.level_states
            .insert(level_id, LevelStatus::Completed { stars: new_stars });

        let next_id = level_id + 1;
        if let Some(LevelStatus::Locked) | None = self.level_states.get(&next_id) {
            if self.total_stars >= stars_to_unlock_next {
                self.level_states.insert(next_id, LevelStatus::Available);
            }
        }
    }

    pub fn is_level_available(&self, level_id: u32) -> bool {
        matches!(
            self.level_states.get(&level_id),
            Some(LevelStatus::Available) | Some(LevelStatus::Completed { .. })
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_profile_unlocks_level_1_and_6() {
        let p = PlayerProfile::new("en");
        assert!(p.is_level_available(1));
        assert!(p.is_level_available(6));
        assert!(!p.is_level_available(2));
    }

    #[test]
    fn test_complete_level_accumulates_stars_and_unlocks_next() {
        let mut p = PlayerProfile::new("en");
        p.complete_level(1, 3, 1);
        assert_eq!(p.total_stars, 3);
        assert!(p.is_level_available(2));
    }

    #[test]
    fn test_replaying_level_does_not_double_count_stars() {
        let mut p = PlayerProfile::new("en");
        p.complete_level(1, 2, 0);
        p.complete_level(1, 3, 0);
        assert_eq!(p.total_stars, 3);
    }
}
