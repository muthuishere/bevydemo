use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use gameplay::app_states::AppState;
use gameplay::events::AnswerSubmittedEvent;
use gameplay::level_plugin::CurrentLevelContext;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin);
        }
        app.add_systems(Update, hud_system.run_if(in_state(AppState::Play)));
    }
}

fn hud_system(
    mut contexts: EguiContexts,
    ctx: Res<CurrentLevelContext>,
    mut answer_events: EventWriter<AnswerSubmittedEvent>,
) {
    let puzzle = match &ctx.current_puzzle {
        Some(p) => p.clone(),
        None => return,
    };
    let op_str = match puzzle.operation {
        core_domain::math_engine::Operation::Addition => "+",
        core_domain::math_engine::Operation::Subtraction => "−",
    };
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            ui.heading(format!(
                "Puzzle {}/{}",
                ctx.puzzles_done + 1,
                ctx.total_puzzles
            ));
            ui.add_space(20.0);
            ui.label(
                egui::RichText::new(format!(
                    "{} {} {} = ?",
                    puzzle.operand_a, op_str, puzzle.operand_b
                ))
                .size(48.0),
            );
            ui.add_space(30.0);
            ui.horizontal(|ui| {
                for &choice in &puzzle.choices {
                    if ui
                        .add(
                            egui::Button::new(
                                egui::RichText::new(choice.to_string()).size(32.0),
                            )
                            .min_size(egui::vec2(80.0, 60.0)),
                        )
                        .clicked()
                    {
                        let correct = puzzle.is_correct(choice);
                        answer_events.send(AnswerSubmittedEvent {
                            answer: choice,
                            correct,
                        });
                    }
                }
            });
            ui.add_space(20.0);
            ui.label(format!(
                "✓ {} | ✗ {} | ⭐ streak {}",
                ctx.score.correct, ctx.score.incorrect, ctx.score.streak
            ));
        });
    });
}
