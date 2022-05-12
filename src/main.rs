use bevy::prelude::*;

mod camera;
mod debug;
mod player;
mod terrain;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "erosion".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(terrain::TerrainPlugin)
        .add_plugin(debug::DebugPlugin)
        .run();
}
