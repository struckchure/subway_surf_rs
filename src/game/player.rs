use avian3d::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component, Clone, Copy, PartialEq)]
pub enum CurrentTrack {
    Left,
    Middle,
    Right,
}

impl CurrentTrack {
    pub fn as_index(&self) -> u8 {
        match self {
            CurrentTrack::Left => 0,
            CurrentTrack::Middle => 1,
            CurrentTrack::Right => 2,
        }
    }
}

#[derive(Component)]
pub struct Velocity {
    pub forward: f32,
}

#[derive(Component, Clone, Copy, PartialEq)]
pub enum AnimationState {
    Running,
    Sliding,
    Jumping,
}

#[derive(Component)]
pub struct SlideTimer {
    pub timer: Timer,
}

// Limb marker components for animation
#[derive(Component)]
pub struct LeftArm;

#[derive(Component)]
pub struct RightArm;

#[derive(Component)]
pub struct LeftLeg;

#[derive(Component)]
pub struct RightLeg;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game_config: Res<crate::resources::game_config::GameConfig>,
) {
    let x_position = 0.0;

    // Colors
    let skin_color = Color::srgb(0.9, 0.75, 0.6);
    let shirt_color = Color::srgb(0.2, 0.5, 0.9);
    let pants_color = Color::srgb(0.2, 0.2, 0.35);
    let shoe_color = Color::srgb(0.15, 0.15, 0.15);

    // Meshes (scaled up for better visibility)
    let head_mesh = meshes.add(Sphere::new(0.2));
    let torso_mesh = meshes.add(Cuboid::new(0.45, 0.5, 0.2));
    let arm_mesh = meshes.add(Cylinder::new(0.07, 0.4));
    let leg_mesh = meshes.add(Cylinder::new(0.09, 0.45));

    // Materials
    let skin_mat = materials.add(skin_color);
    let shirt_mat = materials.add(shirt_color);
    let pants_mat = materials.add(pants_color);

    let mut player_entity = commands.spawn((
        Player,
        CurrentTrack::Middle,
        Velocity {
            forward: game_config.base_speed,
        },
        AnimationState::Running,
        Transform::from_xyz(x_position, 1.5, 0.0),
        RigidBody::Dynamic,
        Collider::capsule(0.8, 0.4),
        LockedAxes::ROTATION_LOCKED,
        LinearVelocity::ZERO,
        GravityScale(2.5),
    ));

    player_entity.with_children(|parent| {
        // Head
        parent.spawn((
            Mesh3d(head_mesh.clone()),
            MeshMaterial3d(skin_mat.clone()),
            Transform::from_xyz(0.0, 0.7, 0.0),
        ));

        // Torso
        parent.spawn((
            Mesh3d(torso_mesh.clone()),
            MeshMaterial3d(shirt_mat.clone()),
            Transform::from_xyz(0.0, 0.25, 0.0),
        ));

        // Left Arm (pivot at shoulder)
        parent.spawn((
            LeftArm,
            Mesh3d(arm_mesh.clone()),
            MeshMaterial3d(shirt_mat.clone()),
            Transform::from_xyz(-0.32, 0.3, 0.0).with_rotation(Quat::from_rotation_z(0.15)),
        ));

        // Right Arm (pivot at shoulder)
        parent.spawn((
            RightArm,
            Mesh3d(arm_mesh.clone()),
            MeshMaterial3d(shirt_mat.clone()),
            Transform::from_xyz(0.32, 0.3, 0.0).with_rotation(Quat::from_rotation_z(-0.15)),
        ));

        // Left Leg (pivot at hip)
        parent.spawn((
            LeftLeg,
            Mesh3d(leg_mesh.clone()),
            MeshMaterial3d(pants_mat.clone()),
            Transform::from_xyz(-0.14, -0.22, 0.0),
        ));

        // Right Leg (pivot at hip)
        parent.spawn((
            RightLeg,
            Mesh3d(leg_mesh.clone()),
            MeshMaterial3d(pants_mat.clone()),
            Transform::from_xyz(0.14, -0.22, 0.0),
        ));
    });
}
