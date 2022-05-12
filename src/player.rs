use bevy::prelude::*;

mod spawn_digger;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_digger::spawn_digger);
    }
}
