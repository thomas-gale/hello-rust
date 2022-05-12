use bevy::prelude::*;

use super::Digger;

pub fn control_digger(
    mut digger_query: Query<(&Digger, &mut Transform)>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (digger, mut transform) = digger_query.single_mut();

    let mut z_delta = 0.0;
    if keys.pressed(KeyCode::W) {
        z_delta -= digger.speed * time.delta_seconds();
    }
    if keys.pressed(KeyCode::S) {
        z_delta += digger.speed * time.delta_seconds();
    }

    let target = transform.translation + Vec3::new(0.0, 0.0, z_delta);
    transform.translation = target;
}
