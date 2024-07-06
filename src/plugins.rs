use crate::systems::{change_bevy_velocity, move_bevy, scale_bevy, scale_pipe, setup};
use bevy::app::{Plugin, Startup, Update};
pub struct FlappyPlugin;

impl Plugin for FlappyPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            (
                move_bevy,
                change_bevy_velocity,
                scale_pipe,
                scale_bevy,
            ),
        );
    }
}
