use bevy::prelude::*;

#[derive(Event, Debug, Clone)]
pub struct LevelSelectedEvent {
    pub level_id: u32,
}

#[derive(Event, Debug, Clone)]
pub struct AnswerSubmittedEvent {
    pub answer: i32,
    pub correct: bool,
}

#[derive(Event, Debug, Clone)]
pub struct LevelCompletedEvent {
    pub level_id: u32,
    pub stars: u8,
}

#[derive(Event, Debug, Clone)]
pub struct LanguageChangedEvent {
    pub language_code: String,
}
