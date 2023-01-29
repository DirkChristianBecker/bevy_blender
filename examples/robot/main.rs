use bevy::prelude::*;
use bevy_blender::*;

// Use pan orbit camera
use bevy_cameras::prelude::*;
use bevy_cameras::prelude::Camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(BlenderPlugin)
        .add_plugin(PanOrbitCamera::default())
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    spawn_blender_object(
        &mut commands,
        &asset_server,
        "robot.blend",
        "Cube.002",
        false,
        None,
    );

    // Light and camera
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });

    let translation = Vec3::new(5.0, 5.0, 5.0);
    let look_at = Vec3::ZERO;
    PanOrbitCamera::init_camera(&mut commands, translation, look_at);
}
