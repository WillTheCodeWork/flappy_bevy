use bevy::{prelude::App, DefaultPlugins};
mod structs;
mod plugins;
mod systems;
mod enums;
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, plugins::FlappyPlugin))
        .run();
}
