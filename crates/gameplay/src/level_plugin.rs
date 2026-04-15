use bevy::prelude::*;
use crate::app_states::AppState;
use crate::events::*;
use core_domain::{
    math_engine::{Puzzle, PuzzleGenerator},
    scoring::SessionScore,
};

#[derive(Resource, Default)]
pub struct CurrentLevelContext {
    pub level_id: u32,
    pub puzzles_done: u32,
    pub total_puzzles: u32,
    pub score: SessionScore,
    pub current_puzzle: Option<Puzzle>,
}

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentLevelContext>()
            .add_event::<LevelSelectedEvent>()
            .add_event::<AnswerSubmittedEvent>()
            .add_event::<LevelCompletedEvent>()
            .add_event::<LanguageChangedEvent>()
            .add_systems(OnEnter(AppState::Play), setup_level)
            .add_systems(
                Update,
                (handle_answer, check_level_complete).run_if(in_state(AppState::Play)),
            );
    }
}

fn setup_level(mut ctx: ResMut<CurrentLevelContext>) {
    ctx.puzzles_done = 0;
    ctx.score = SessionScore::default();
}

fn handle_answer(
    mut events: EventReader<AnswerSubmittedEvent>,
    mut ctx: ResMut<CurrentLevelContext>,
) {
    for ev in events.read() {
        ctx.score.record_answer(ev.correct);
        ctx.puzzles_done += 1;
    }
}

fn check_level_complete(
    ctx: Res<CurrentLevelContext>,
    mut completed: EventWriter<LevelCompletedEvent>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if ctx.puzzles_done >= ctx.total_puzzles && ctx.total_puzzles > 0 {
        let stars = ctx.score.stars(0.8);
        completed.send(LevelCompletedEvent {
            level_id: ctx.level_id,
            stars,
        });
        next_state.set(AppState::Results);
    }
}

// Keep PuzzleGenerator in scope to suppress unused-import warning
const _: fn() = || {
    let _ = PuzzleGenerator::generate;
};
