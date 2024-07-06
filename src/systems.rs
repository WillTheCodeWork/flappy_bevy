use bevy::{
    asset::AssetServer,
    input::ButtonInput,
    prelude::{default, Camera2dBundle, Commands, KeyCode, Query, Res, With},
    sprite::SpriteBundle,
    time::Time,
    transform::components::Transform,
};

use crate::components::{Bevy, Velocity};

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("bevy_logo.png"),
            transform: Transform::from_xyz(100.0, 0.0, 0.0),
            ..default()
        },
        Bevy,
        Velocity(0.0),
    ));
}
pub fn change_bevy_velocity(mut velocity_query: Query<&mut Velocity, With<Bevy>>, time: Res<Time>) {
    for mut velocity in &mut velocity_query {
        velocity.0 -= 5.0;
    }
}
pub fn move_bevy(
    time: Res<Time>,
    mut bevy_position: Query<(&mut Transform, &mut Velocity)>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    for (mut transform, mut velocity) in &mut bevy_position {
        transform.translation.y += velocity.0 * time.delta_seconds();
        if keyboard.pressed(KeyCode::Space) {
            if velocity.0 < 0.0 {
                velocity.0 = 0.0;
            }
            velocity.0 += 30.0;
        }
    }
}
