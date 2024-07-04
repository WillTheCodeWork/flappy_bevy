use bevy::{prelude::App, DefaultPlugins};
mod systems;
mod plugins;
mod resources;
mod components;
fn main() {
    App::new()
    .add_plugins((DefaultPlugins, plugins::FlappyPlugin))
    .run();
}