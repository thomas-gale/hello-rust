use bevy::prelude::*;

mod control_digger;
mod digger;
mod spawn_digger;

pub use digger::Digger;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_digger::spawn_digger)
            .add_system(control_digger::control_digger);
    }
}
