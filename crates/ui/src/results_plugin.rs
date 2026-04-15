use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use gameplay::app_states::AppState;
use gameplay::level_plugin::CurrentLevelContext;

pub struct ResultsPlugin;

impl Plugin for ResultsPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin);
        }
        app.add_systems(Update, results_system.run_if(in_state(AppState::Results)));
    }
}

fn results_system(
    mut contexts: EguiContexts,
    ctx: Res<CurrentLevelContext>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let stars = ctx.score.stars(0.8);
    let star_display: String = (0..3)
        .map(|i| if i < stars { "⭐" } else { "☆" })
        .collect();
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(60.0);
            ui.heading("Level Complete!");
            ui.add_space(20.0);
            ui.label(egui::RichText::new(&star_display).size(64.0));
            ui.add_space(10.0);
            ui.label(format!(
                "Correct: {} | Wrong: {}",
                ctx.score.correct, ctx.score.incorrect
            ));
            ui.label(format!("Max Streak: {}", ctx.score.max_streak));
            ui.add_space(30.0);
            if ui
                .add(
                    egui::Button::new("Back to Menu").min_size(egui::vec2(160.0, 50.0)),
                )
                .clicked()
            {
                next_state.set(AppState::MainMenu);
            }
        });
    });
}
