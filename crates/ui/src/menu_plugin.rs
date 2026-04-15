use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use gameplay::app_states::AppState;
use gameplay::events::LevelSelectedEvent;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin);
        }
        app.add_systems(Update, menu_system.run_if(in_state(AppState::MainMenu)));
    }
}

fn menu_system(
    mut contexts: EguiContexts,
    mut level_events: EventWriter<LevelSelectedEvent>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(40.0);
            ui.heading(egui::RichText::new("🧮 MathQuest").size(48.0));
            ui.label("Learn Addition & Subtraction through adventure!");
            ui.add_space(30.0);
            ui.label(egui::RichText::new("➕ Addition Levels").size(24.0).strong());
            ui.horizontal(|ui| {
                for i in 1..=5u32 {
                    if ui
                        .add(
                            egui::Button::new(
                                egui::RichText::new(format!("Level {}", i)).size(20.0),
                            )
                            .min_size(egui::vec2(100.0, 60.0)),
                        )
                        .clicked()
                    {
                        level_events.send(LevelSelectedEvent { level_id: i });
                        next_state.set(AppState::LevelIntro);
                    }
                }
            });
            ui.add_space(20.0);
            ui.label(
                egui::RichText::new("➖ Subtraction Levels")
                    .size(24.0)
                    .strong(),
            );
            ui.horizontal(|ui| {
                for i in 6..=10u32 {
                    if ui
                        .add(
                            egui::Button::new(
                                egui::RichText::new(format!("Level {}", i)).size(20.0),
                            )
                            .min_size(egui::vec2(100.0, 60.0)),
                        )
                        .clicked()
                    {
                        level_events.send(LevelSelectedEvent { level_id: i });
                        next_state.set(AppState::LevelIntro);
                    }
                }
            });
        });
    });
}
