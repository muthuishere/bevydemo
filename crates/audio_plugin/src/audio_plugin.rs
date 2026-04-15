use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use gameplay::app_states::AppState;
use gameplay::events::AnswerSubmittedEvent;

pub struct MathQuestAudioPlugin;

impl Plugin for MathQuestAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin);
        app.add_systems(
            Update,
            play_answer_feedback.run_if(in_state(AppState::Play)),
        );
    }
}

fn play_answer_feedback(
    mut events: EventReader<AnswerSubmittedEvent>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    for ev in events.read() {
        let sfx = if ev.correct {
            "audio/sfx/correct.ogg"
        } else {
            "audio/sfx/wrong.ogg"
        };
        audio.play(asset_server.load(sfx));
    }
}
