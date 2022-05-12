use bevy::prelude::*;

use super::Digger;

pub fn control_digger(
    mut digger_query: Query<(&Digger, &mut Transform)>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (digger, mut transform) = digger_query.single_mut();

    // let mut x_delta: f32 = 0.0;
    let mut z_delta = 0.0;
    let mut y_rotation: f32 = 0.0;

    if keys.pressed(KeyCode::A) {
        y_rotation += digger.speed * 0.5 * time.delta_seconds();
    }
    if keys.pressed(KeyCode::D) {
        y_rotation -= digger.speed * 0.5 * time.delta_seconds();
    }
    if keys.pressed(KeyCode::W) {
        z_delta -= digger.speed * time.delta_seconds();
    }
    if keys.pressed(KeyCode::S) {
        z_delta += digger.speed * time.delta_seconds();
    }

    let rotation = transform.rotation * Quat::from_rotation_y(y_rotation);
    let target = transform.translation + rotation.mul_vec3(Vec3::new(0.0, 0.0, z_delta));
    transform.rotation = rotation;
    transform.translation = target;
}
