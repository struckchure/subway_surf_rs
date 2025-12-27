use crate::game::player::Player;
use crate::resources::game_config::GameConfig;
use avian3d::prelude::*;
use bevy::prelude::*;

#[derive(Component, Clone, Copy, PartialEq)]
pub enum TrainType {
    Stationary,         // Just sits there, must go around
    StationaryWithRamp, // Has a ramp, can climb on top
    Moving,             // Moves along the track, must avoid
}

#[derive(Component)]
pub struct Train {
    pub track_index: u8,
    pub train_type: TrainType,
    pub length: f32, // Length of the train
    pub speed: f32,  // Speed for moving trains (0 for stationary)
}

#[derive(Component)]
pub struct TrainRamp;

#[derive(Component)]
pub struct TrainTop {
    pub height: f32,
}

/// Marker for train-related entities that should be recycled with trains
#[derive(Component)]
pub struct TrainPart;

// Simple pseudo-random for train spawning
fn train_random(seed: u32) -> u32 {
    let mut x = seed;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    x
}

pub fn spawn_train(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    track_index: u8,
    z_position: f32,
    train_type: TrainType,
    game_config: &GameConfig,
) -> Entity {
    let x_offset = (track_index as f32 - 1.0) * game_config.track_spacing;

    // Train colors
    let train_body_color = materials.add(Color::srgb(0.7, 0.1, 0.15)); // Dark red
    let train_stripe_color = materials.add(Color::srgb(0.95, 0.85, 0.2)); // Yellow stripe
    let train_window_color = materials.add(Color::srgb(0.2, 0.3, 0.4)); // Dark blue-gray windows
    let train_roof_color = materials.add(Color::srgb(0.5, 0.5, 0.55)); // Gray roof
    let wheel_color = materials.add(Color::srgb(0.15, 0.15, 0.15)); // Black wheels
    let ramp_color = materials.add(Color::srgb(0.6, 0.5, 0.3)); // Wooden ramp

    // Train dimensions
    let train_length = 8.0;
    let train_width = 1.6;
    let train_height = 2.2;
    let train_y_base = train_height / 2.0;

    // Meshes
    let body_mesh = meshes.add(Cuboid::new(train_width, train_height, train_length));
    let roof_mesh = meshes.add(Cuboid::new(train_width - 0.1, 0.15, train_length - 0.2));
    let stripe_mesh = meshes.add(Cuboid::new(train_width + 0.02, 0.2, train_length + 0.02));
    let window_mesh = meshes.add(Cuboid::new(0.02, 0.5, 0.8));
    let wheel_mesh = meshes.add(Cylinder::new(0.25, 0.15));

    let speed = match train_type {
        TrainType::Moving => game_config.current_speed * 0.7, // Slightly slower than player
        _ => 0.0,
    };

    // Use Kinematic for moving trains (so collider follows transform), Static for stationary
    let rigid_body = match train_type {
        TrainType::Moving => RigidBody::Kinematic,
        _ => RigidBody::Static,
    };

    let mut train_entity = commands.spawn((
        Train {
            track_index,
            train_type,
            length: train_length,
            speed,
        },
        Transform::from_translation(Vec3::new(x_offset, 0.0, z_position)),
        Visibility::default(),
        // Add rigid body physics for physical collision
        rigid_body,
    ));

    train_entity.with_children(|parent| {
        // Train body collider (positioned at train body center)
        parent.spawn((
            Collider::cuboid(train_width / 2.0, train_height / 2.0, train_length / 2.0),
            Transform::from_xyz(0.0, train_y_base, 0.0),
        ));

        // Train top collider - full length platform on top for walking
        parent.spawn((
            Collider::cuboid(train_width / 2.0, 0.1, train_length / 2.0),
            Transform::from_xyz(0.0, train_height + 0.1, 0.0),
        ));

        // Main body (visual)
        parent.spawn((
            Mesh3d(body_mesh.clone()),
            MeshMaterial3d(train_body_color.clone()),
            Transform::from_xyz(0.0, train_y_base, 0.0),
        ));

        // Roof
        parent.spawn((
            Mesh3d(roof_mesh.clone()),
            MeshMaterial3d(train_roof_color.clone()),
            Transform::from_xyz(0.0, train_height + 0.075, 0.0),
        ));

        // Yellow stripe along the side
        parent.spawn((
            Mesh3d(stripe_mesh.clone()),
            MeshMaterial3d(train_stripe_color.clone()),
            Transform::from_xyz(0.0, train_y_base - 0.3, 0.0),
        ));

        // Windows on both sides
        for side in [-1.0, 1.0] {
            for i in 0..4 {
                let window_z = (i as f32 - 1.5) * 1.8;
                parent.spawn((
                    Mesh3d(window_mesh.clone()),
                    MeshMaterial3d(train_window_color.clone()),
                    Transform::from_xyz(
                        side * (train_width / 2.0 + 0.01),
                        train_y_base + 0.4,
                        window_z,
                    ),
                ));
            }
        }

        // Wheels
        for side in [-1.0, 1.0] {
            for i in [-1.0, 1.0] {
                parent.spawn((
                    Mesh3d(wheel_mesh.clone()),
                    MeshMaterial3d(wheel_color.clone()),
                    Transform::from_xyz(side * 0.6, 0.25, i * 2.5)
                        .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
                ));
            }
        }
    });

    // Get the train entity ID before spawning more entities
    let train_id = train_entity.id();

    // Spawn ramp and platform as separate physics entities (not children)
    if train_type == TrainType::StationaryWithRamp {
        let ramp_length = 6.0;
        let ramp_width = 1.6;
        let ramp_thickness = 0.2;

        // Calculate ramp angle to reach train top height
        let ramp_rise = train_height + 0.1; // Match the train top platform height
        let ramp_angle: f32 = (ramp_rise / ramp_length).atan(); // Calculate angle based on rise/run

        let ramp_center_y = ramp_rise / 2.0;
        let ramp_center_z = z_position - train_length / 2.0 - (ramp_length / 2.0);

        let ramp_mesh = meshes.add(Cuboid::new(ramp_width, ramp_thickness, ramp_length));

        // Side rails for the ramp (visual only)
        let rail_mesh = meshes.add(Cuboid::new(0.08, 0.25, ramp_length));
        let rail_color = materials.add(Color::srgb(0.4, 0.35, 0.25));

        // Visual ramp mesh
        commands.spawn((
            TrainRamp,
            Mesh3d(ramp_mesh),
            MeshMaterial3d(ramp_color.clone()),
            Transform::from_xyz(x_offset, ramp_center_y, ramp_center_z)
                .with_rotation(Quat::from_rotation_x(-ramp_angle)),
        ));

        // Create smooth stepped colliders - more steps for smoother climbing
        let num_steps = 20;
        let step_length = ramp_length / num_steps as f32;
        let step_height = ramp_rise / num_steps as f32;
        let ramp_start_z = z_position - train_length / 2.0 - ramp_length;

        for i in 0..num_steps {
            let step_y = (i as f32 + 0.5) * step_height;
            let step_z = ramp_start_z + (i as f32 + 0.5) * step_length;

            // Each step slightly overlaps with the next for smooth transition
            commands.spawn((
                TrainPart,
                RigidBody::Static,
                Collider::cuboid(
                    ramp_width / 2.0,
                    step_height / 2.0 + 0.05,
                    step_length / 2.0 + 0.1,
                ),
                Transform::from_xyz(x_offset, step_y, step_z),
            ));
        }

        // Add a bridge collider connecting ramp top to train top platform
        commands.spawn((
            TrainPart,
            RigidBody::Static,
            Collider::cuboid(ramp_width / 2.0, 0.1, 0.5),
            Transform::from_xyz(x_offset, ramp_rise, z_position - train_length / 2.0 - 0.25),
        ));

        // Left rail
        commands.spawn((
            TrainPart,
            Mesh3d(rail_mesh.clone()),
            MeshMaterial3d(rail_color.clone()),
            Transform::from_xyz(
                x_offset - (ramp_width / 2.0 + 0.04),
                ramp_center_y + 0.12,
                ramp_center_z,
            )
            .with_rotation(Quat::from_rotation_x(-ramp_angle)),
        ));

        // Right rail
        commands.spawn((
            TrainPart,
            Mesh3d(rail_mesh),
            MeshMaterial3d(rail_color),
            Transform::from_xyz(
                x_offset + (ramp_width / 2.0 + 0.04),
                ramp_center_y + 0.12,
                ramp_center_z,
            )
            .with_rotation(Quat::from_rotation_x(-ramp_angle)),
        ));
    }

    // ALL trains get a platform on top so player can jump onto any train
    // Note: Collider::cuboid uses HALF-EXTENTS
    let platform_width = train_width - 0.2;
    let platform_height = 0.15;
    let platform_length = train_length; // Full length to avoid gaps
    let top_platform_mesh = meshes.add(Cuboid::new(
        platform_width,
        platform_height,
        platform_length,
    ));
    commands.spawn((
        TrainTop {
            height: train_height + 0.1,
        },
        Mesh3d(top_platform_mesh),
        MeshMaterial3d(train_roof_color.clone()),
        Transform::from_xyz(x_offset, train_height + 0.025, z_position),
        RigidBody::Static,
        Collider::cuboid(
            platform_width / 2.0,
            platform_height / 2.0,
            platform_length / 2.0,
        ),
    ));

    train_id
}

pub fn generate_trains_procedurally(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game_config: Res<GameConfig>,
    train_query: Query<&Transform, With<Train>>,
    player_query: Query<&Transform, (With<Player>, Without<Train>)>,
) {
    if let Ok(player_transform) = player_query.single() {
        let player_z = player_transform.translation.z;

        // Don't spawn trains too early
        if player_z < 50.0 {
            return;
        }

        // Minimum spawn distance ahead of player
        let min_spawn_distance = 80.0;
        // Maximum spawn distance ahead
        let max_spawn_distance = 150.0;
        // Minimum gap between trains
        let min_train_gap = 40.0;

        // Find the furthest train
        let mut max_train_z = player_z;
        let mut train_count: u32 = 0;
        for transform in train_query.iter() {
            train_count += 1;
            if transform.translation.z > max_train_z {
                max_train_z = transform.translation.z;
            }
        }

        // Only spawn if there's room and we need more trains ahead
        let spawn_threshold = player_z + min_spawn_distance;

        if max_train_z < spawn_threshold {
            let target_z = player_z + min_spawn_distance + 20.0;

            let seed = train_random(
                train_count
                    .wrapping_add((target_z * 2.3) as u32)
                    .wrapping_add(0xBEEFCAFE),
            );

            // Pick a track (0, 1, or 2)
            let track_index = (seed % 3) as u8;

            // Determine train type
            let train_type = match (seed / 3) % 5 {
                0 => TrainType::StationaryWithRamp, // 20% chance - climbable
                1 | 2 => TrainType::Stationary,     // 40% chance - must avoid
                _ => TrainType::Moving,             // 40% chance - moving
            };

            spawn_train(
                &mut commands,
                &mut meshes,
                &mut materials,
                track_index,
                target_z,
                train_type,
                &game_config,
            );
        } else if max_train_z < player_z + max_spawn_distance {
            let next_z = max_train_z + min_train_gap;

            if next_z < player_z + max_spawn_distance {
                let seed = train_random(
                    train_count
                        .wrapping_mul(23)
                        .wrapping_add((next_z * 4.1) as u32)
                        .wrapping_add(0xDEADFACE),
                );

                let track_index = (seed % 3) as u8;

                let train_type = match (seed / 3) % 5 {
                    0 => TrainType::StationaryWithRamp,
                    1 | 2 => TrainType::Stationary,
                    _ => TrainType::Moving,
                };

                spawn_train(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    track_index,
                    next_z,
                    train_type,
                    &game_config,
                );
            }
        }
    }
}

pub fn move_trains(mut train_query: Query<(Entity, &Train, &mut Transform)>, time: Res<Time>) {
    // First pass: collect all train positions
    let train_positions: Vec<(Entity, u8, f32, f32)> = train_query
        .iter()
        .map(|(entity, train, transform)| {
            (
                entity,
                train.track_index,
                transform.translation.z,
                train.length,
            )
        })
        .collect();

    // Second pass: move trains that aren't blocked
    for (entity, train, mut transform) in train_query.iter_mut() {
        if train.train_type == TrainType::Moving {
            let my_track = train.track_index;
            let my_z = transform.translation.z;
            let my_back = my_z - train.length / 2.0;

            // Check if any other train is blocking us (in front, same track)
            let min_gap = 2.0; // Minimum gap between trains
            let mut blocked = false;

            for (other_entity, other_track, other_z, other_length) in &train_positions {
                // Skip self
                if *other_entity == entity {
                    continue;
                }

                if *other_track == my_track {
                    let other_front = other_z + other_length / 2.0;
                    // Check if this train is in front of us (lower Z since we move toward negative Z)
                    // A train is "in front" if its front edge is less than our back edge
                    if other_front < my_back {
                        let gap = my_back - other_front;
                        if gap < min_gap + train.speed * time.delta_secs() {
                            blocked = true;
                            break;
                        }
                    }
                }
            }

            if !blocked {
                // Moving trains come towards the player (negative Z direction)
                transform.translation.z -= train.speed * time.delta_secs();
            }
        }
    }
}

pub fn recycle_trains(
    mut commands: Commands,
    train_query: Query<(Entity, &Transform), With<Train>>,
    ramp_query: Query<(Entity, &Transform), (With<TrainRamp>, Without<Train>)>,
    top_query: Query<(Entity, &Transform), (With<TrainTop>, Without<Train>, Without<TrainRamp>)>,
    parts_query: Query<
        (Entity, &Transform),
        (
            With<TrainPart>,
            Without<Train>,
            Without<TrainRamp>,
            Without<TrainTop>,
        ),
    >,
    camera_query: Query<
        &Transform,
        (
            With<Camera3d>,
            Without<Train>,
            Without<TrainRamp>,
            Without<TrainTop>,
            Without<TrainPart>,
        ),
    >,
) {
    if let Ok(camera_transform) = camera_query.single() {
        let camera_z = camera_transform.translation.z;

        // Recycle trains
        for (entity, transform) in train_query.iter() {
            if transform.translation.z < camera_z - 30.0 {
                commands.entity(entity).despawn();
            }
        }

        // Recycle ramps
        for (entity, transform) in ramp_query.iter() {
            if transform.translation.z < camera_z - 35.0 {
                commands.entity(entity).despawn();
            }
        }

        // Recycle train tops
        for (entity, transform) in top_query.iter() {
            if transform.translation.z < camera_z - 30.0 {
                commands.entity(entity).despawn();
            }
        }

        // Recycle train parts (step colliders, rails, etc.)
        for (entity, transform) in parts_query.iter() {
            if transform.translation.z < camera_z - 35.0 {
                commands.entity(entity).despawn();
            }
        }
    }
}
