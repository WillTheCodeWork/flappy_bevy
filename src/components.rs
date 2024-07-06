use bevy::prelude::Component;

#[derive(Component)]
pub enum Direction {
    Up,
    Down,
}
#[derive(Component)]
pub struct Bevy;
#[derive(Component)]
pub struct Velocity(i32);
