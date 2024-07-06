use bevy::{prelude::App, DefaultPlugins};
mod components;
mod plugins;
mod systems;
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, plugins::FlappyPlugin))
        .run();
}
