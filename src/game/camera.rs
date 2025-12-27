use crate::game::player::Player;
use bevy::prelude::*;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, -10.0).looking_at(Vec3::new(0.0, 0.0, 10.0), Vec3::Y),
    ));

    // Directional light (sun)
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.8, 0.2, 0.0)),
    ));

    // Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 500.0,
        affects_lightmapped_meshes: true,
    });
}

pub fn follow_player(
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Camera3d>)>,
) {
    if let Ok(player_transform) = player_query.single() {
        if let Ok(mut camera_transform) = camera_query.single_mut() {
            let player_z = player_transform.translation.z;
            // Keep camera behind player
            camera_transform.translation.z = player_z - 10.0;
            // Look at a point ahead of the player
            let look_target = Vec3::new(0.0, 0.5, player_z + 5.0);
            camera_transform.look_at(look_target, Vec3::Y);
        }
    }
}

pub fn handle_viewport_resize(mut windows: Query<&mut Window>) {
    for mut window in windows.iter_mut() {
        if window.width() < 800.0 || window.height() < 600.0 {
            window.resolution.set(800.0, 600.0);
        }
    }
}
