use bevy::{
    asset::AssetServer,
    input::ButtonInput,
    prelude::{default, Camera2dBundle, Commands, KeyCode, Query, Res, With},
    sprite::SpriteBundle,
    time::Time,
    transform::components::Transform,
    window::{EnabledButtons, PrimaryWindow, Window},
};

use crate::structs::{Bevy, Pipe, Velocity};

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    //bevy berb
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("bevy_logo.png"),
            ..default()
        },
        Bevy,
        Velocity(0.0),
    ));
    //pipe
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("pipe.png"),
            transform: Transform::from_xyz(0.0, -300.0, 0.0),
            ..default()
        },
        Pipe,
    ));
}
pub fn change_bevy_velocity(mut velocity_query: Query<&mut Velocity, With<Bevy>>) {
    for mut velocity in &mut velocity_query {
        velocity.0 -= 5.0;
    }
}
pub fn move_bevy(
    time: Res<Time>,
    //this query looks complicated, but it isn't
    mut bevy_position: Query<(&mut Transform, &mut Velocity), With<Bevy>>,
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
pub fn scale_pipe(mut pipe_query: Query<&mut Transform, With<Pipe>>) {
    for mut pipe in &mut pipe_query {
        pipe.scale.y = 2.3;
    }
}
pub fn scale_bevy(mut bevy_query: Query<&mut Transform, With<Bevy>>) {
    for mut bevy in &mut bevy_query {
        bevy.scale.y = 0.5;
        bevy.scale.x = 0.5;
    }
}
pub fn customize_window(mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    for mut window in &mut window_query {
        window.title = "test".into();
        window.name = Some("me.will.flappy_bevy".into());
        window.enabled_buttons = EnabledButtons {
            close: true,
            minimize: true,
            maximize: false,
        };
        window.resizable = false;
    }
}
pub fn move_pipe(mut pipe_query: Query<&mut Transform, With<Pipe>>, time: Res<Time>) {
    for mut pipe in &mut pipe_query {
        pipe.translation.x -= 150 * time.delta_seconds();
    }
}
