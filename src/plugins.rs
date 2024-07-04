use bevy::app::{Plugin, Startup, Update};
use crate::systems::{move_bevy, setup};
pub struct FlappyPlugin;

impl Plugin for FlappyPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, move_bevy);
    }
}