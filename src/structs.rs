use bevy::prelude::{Component, Resource};

#[derive(Component)]
pub struct Bevy;
#[derive(Component)]
pub struct Pipe;
#[derive(Component)]
pub struct Velocity(pub f32);
#[derive(Resource)]
pub struct Score(u32);