use bevy::{prelude::App, DefaultPlugins};
mod components;
mod plugins;
mod resources;
mod systems;
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, plugins::FlappyPlugin))
        .run();
}
