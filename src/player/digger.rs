use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Component, Inspectable)]
pub struct Digger {
    pub speed: f32,
}
