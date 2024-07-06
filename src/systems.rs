use bevy::{
    asset::AssetServer,
    input::ButtonInput,
    prelude::{default, Camera2dBundle, Commands, KeyCode, Query, Res},
    sprite::SpriteBundle,
    time::Time,
    transform::components::Transform,
};

use crate::components::{Bevy, Direction, Velocity};

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("bevy_logo.png"),
            transform: Transform::from_xyz(100.0, 0.0, 0.0),
            ..default()
        },
        Direction::Down,
    ));
    commands.spawn((Bevy, Velocity(0)));
}
pub fn change_bevy_velocity(mut velocity_query: Query<&mut Velocity>) {
    for mut velocity in &mut velocity_query {
        velocity.0 -= 1;
    }
}
pub fn move_bevy(
    time: Res<Time>,
    mut bevy_position: Query<(&mut Direction, &mut Transform)>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    for (mut bevy, mut transform) in &mut bevy_position {
        match *bevy {
            Direction::Up => transform.translation.y += 150.0 * time.delta_seconds(),
            Direction::Down => transform.translation.y -= 150.0 * time.delta_seconds(),
        }
        if keyboard.pressed(KeyCode::Space) {
            *bevy = Direction::Up
        }
    }
}
