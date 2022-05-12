use bevy::prelude::*;

use crate::player::Digger;

pub fn spawn_digger(mut commands: Commands, asset_server: Res<AssetServer>) {
    let digger_gltf = asset_server.load("models/digger_v3.glb#Scene0");

    commands
        .spawn_bundle(TransformBundle {
            local: Transform::from_xyz(0., 0., 0.),
            global: GlobalTransform::identity(),
        })
        .with_children(|parent| {
            parent.spawn_scene(digger_gltf);
        })
        .insert(Name::new("player_digger"))
        .insert(Digger {});
}
