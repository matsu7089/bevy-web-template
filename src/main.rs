use bevy::prelude::*;

const SCREEN_WIDTH: f32 = 500.0;
const SCREEN_HEIGHT: f32 = 500.0;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            resizable: false,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .run();
}
