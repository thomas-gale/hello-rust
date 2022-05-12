use bevy::prelude::*;

pub fn spawn_digger(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_scene(asset_server.load("models/digger_v3.glb#Scene0"));
}
