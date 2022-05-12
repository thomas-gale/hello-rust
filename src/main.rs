use bevy::prelude::*;

mod camera;
mod player;
mod terrain;

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(terrain::TerrainPlugin)
        .add_startup_system(set_window)
        .add_system(animate_light_direction)
        .run();
}

fn set_window(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    println!("Window size was: {},{}", window.width(), window.height());
    window.set_resolution(500.0, 500.0);
    window.set_title("erosion".to_string());
}

fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in query.iter_mut() {
        transform.rotation = Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            time.seconds_since_startup() as f32 * std::f32::consts::TAU / 10.0,
            -std::f32::consts::FRAC_PI_4,
        );
    }
}

// This is the struct that will be passed to your shader
