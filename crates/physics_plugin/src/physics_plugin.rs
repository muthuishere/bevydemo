use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct MathQuestPhysicsPlugin;

impl Plugin for MathQuestPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));
        // Physics-based reward animations: falling stars, bouncing numbers
    }
}
