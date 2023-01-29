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
    // Spawn Blender object with children
    // The parent object's transform is provided
    spawn_blender_object(
        &mut commands,
        &asset_server,
        "demo.blend",
        "SuzanneNoMat",
        true,
        Some(Transform::from_matrix(
            Mat4::from_scale_rotation_translation(
                Vec3::new(0.5, 0.5, 0.5),
                Quat::IDENTITY,
                Vec3::new(-2.0, 0.0, 0.0),
            ),
        )),
    );

    spawn_blender_object(
        &mut commands,
        &asset_server,
        "demo.blend",
        "SuzanneNodesMat",
        true,
        Some(Transform::from_matrix(
            Mat4::from_scale_rotation_translation(
                Vec3::new(0.5, 0.5, 0.5),
                Quat::IDENTITY,
                Vec3::new(2.0, 0.0, 0.0),
            ),
        )),
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
