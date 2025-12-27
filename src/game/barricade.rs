use crate::game::player::Player;
use crate::game::train::{Train, TrainType};
use crate::resources::game_config::GameConfig;
use bevy::prelude::*;

#[derive(Component, Clone, Copy, PartialEq)]
pub enum ObstacleType {
    JumpOver,   // Low obstacle - must jump
    SlideUnder, // High obstacle - must slide
}

#[derive(Component)]
pub struct Barricade {
    pub track_index: u8,
    pub obstacle_type: ObstacleType,
}

#[derive(Component)]
pub struct CollisionShape {
    pub size: Vec3,
}

pub fn spawn_obstacle(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    track_index: u8,
    z_position: f32,
    obstacle_type: ObstacleType,
    game_config: &GameConfig,
) -> Entity {
    let x_offset = (track_index as f32 - 1.0) * game_config.track_spacing;

    match obstacle_type {
        ObstacleType::JumpOver => {
            // Construction barrier with alternating red and cream upright blocks
            let red_color = materials.add(Color::srgb(0.9, 0.2, 0.15)); // Bright red
            let cream_color = materials.add(Color::srgb(0.95, 0.9, 0.75)); // Cream/tan

            // Upright block segments
            let block_mesh = meshes.add(Cuboid::new(0.18, 0.5, 0.12));
            let leg_mesh = meshes.add(Cuboid::new(0.15, 0.45, 0.15));

            commands
                .spawn((
                    Barricade {
                        track_index,
                        obstacle_type,
                    },
                    CollisionShape {
                        size: Vec3::new(1.5, 0.7, 0.3),
                    },
                    Transform::from_translation(Vec3::new(x_offset, 0.0, z_position)),
                    Visibility::default(),
                ))
                .with_children(|parent| {
                    // Alternating red and cream upright blocks
                    let num_blocks = 8;
                    let total_width = 1.5;
                    let block_spacing = total_width / num_blocks as f32;

                    for i in 0..num_blocks {
                        let x_pos = -total_width / 2.0 + block_spacing * (i as f32 + 0.5);
                        let color = if i % 2 == 0 {
                            red_color.clone()
                        } else {
                            cream_color.clone()
                        };
                        parent.spawn((
                            Mesh3d(block_mesh.clone()),
                            MeshMaterial3d(color),
                            Transform::from_xyz(x_pos, 0.45, 0.0), // No rotation - upright
                        ));
                    }

                    // Left leg (cream colored)
                    parent.spawn((
                        Mesh3d(leg_mesh.clone()),
                        MeshMaterial3d(cream_color.clone()),
                        Transform::from_xyz(-0.65, 0.22, 0.0),
                    ));
                    // Right leg (cream colored)
                    parent.spawn((
                        Mesh3d(leg_mesh.clone()),
                        MeshMaterial3d(cream_color.clone()),
                        Transform::from_xyz(0.65, 0.22, 0.0),
                    ));
                })
                .id()
        }
        ObstacleType::SlideUnder => {
            // Overhead barrier with alternating red/cream upright blocks
            let red_color = materials.add(Color::srgb(0.9, 0.2, 0.15)); // Bright red
            let cream_color = materials.add(Color::srgb(0.95, 0.9, 0.75)); // Cream/tan
            let light_color = materials.add(Color::srgb(1.0, 0.85, 0.1)); // Yellow warning light

            let block_mesh = meshes.add(Cuboid::new(0.18, 0.55, 0.1));
            let pole_mesh = meshes.add(Cuboid::new(0.15, 1.4, 0.15));
            let light_mesh = meshes.add(Sphere::new(0.1));

            commands
                .spawn((
                    Barricade {
                        track_index,
                        obstacle_type,
                    },
                    CollisionShape {
                        size: Vec3::new(1.5, 0.7, 0.3),
                    },
                    Transform::from_translation(Vec3::new(x_offset, 0.0, z_position)),
                    Visibility::default(),
                ))
                .with_children(|parent| {
                    // Alternating red and cream upright blocks on overhead sign
                    let num_blocks = 8;
                    let total_width = 1.5;
                    let block_spacing = total_width / num_blocks as f32;

                    for i in 0..num_blocks {
                        let x_pos = -total_width / 2.0 + block_spacing * (i as f32 + 0.5);
                        let color = if i % 2 == 0 {
                            red_color.clone()
                        } else {
                            cream_color.clone()
                        };
                        parent.spawn((
                            Mesh3d(block_mesh.clone()),
                            MeshMaterial3d(color),
                            Transform::from_xyz(x_pos, 1.5, 0.0), // No rotation - upright
                        ));
                    }

                    // Left pole (cream colored)
                    parent.spawn((
                        Mesh3d(pole_mesh.clone()),
                        MeshMaterial3d(cream_color.clone()),
                        Transform::from_xyz(-0.65, 0.7, 0.0),
                    ));
                    // Right pole (cream colored)
                    parent.spawn((
                        Mesh3d(pole_mesh.clone()),
                        MeshMaterial3d(cream_color.clone()),
                        Transform::from_xyz(0.65, 0.7, 0.0),
                    ));
                    // Warning lights on top
                    parent.spawn((
                        Mesh3d(light_mesh.clone()),
                        MeshMaterial3d(light_color.clone()),
                        Transform::from_xyz(-0.5, 1.85, 0.0),
                    ));
                    parent.spawn((
                        Mesh3d(light_mesh.clone()),
                        MeshMaterial3d(light_color.clone()),
                        Transform::from_xyz(0.5, 1.85, 0.0),
                    ));
                })
                .id()
        }
    }
}

// Simple pseudo-random number generator for consistent track distribution
fn pseudo_random(seed: u32) -> u32 {
    let mut x = seed;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    x
}

pub fn generate_obstacles_procedurally(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game_config: Res<GameConfig>,
    barricade_query: Query<&Transform, With<Barricade>>,
    player_query: Query<&Transform, (With<Player>, Without<Barricade>, Without<Train>)>,
    train_query: Query<(&Transform, &Train), Without<Barricade>>,
) {
    if let Ok(player_transform) = player_query.single() {
        let player_z = player_transform.translation.z;

        // Don't spawn obstacles until player has moved forward a bit (grace period at start)
        if player_z < 20.0 {
            return;
        }

        // Minimum spawn distance ahead of player
        let min_spawn_distance = 50.0;
        // Maximum spawn distance ahead
        let max_spawn_distance = 100.0;
        // Minimum gap between obstacles
        let min_obstacle_gap = 20.0;

        // Find the furthest barricade
        let mut max_barricade_z = player_z;
        let mut obstacle_count: u32 = 0;
        for transform in barricade_query.iter() {
            obstacle_count += 1;
            if transform.translation.z > max_barricade_z {
                max_barricade_z = transform.translation.z;
            }
        }

        // Helper function to check if a position conflicts with a ramped train
        let conflicts_with_ramp = |track_index: u8, z_pos: f32| -> bool {
            for (train_transform, train) in train_query.iter() {
                if train.train_type == TrainType::StationaryWithRamp
                    && train.track_index == track_index
                {
                    let train_z = train_transform.translation.z;
                    let ramp_length = 6.0;
                    let train_length = train.length;
                    // Extended buffer before ramp (20 units) so player can see ramp clearly
                    let ramp_start = train_z - train_length / 2.0 - ramp_length - 20.0;
                    // Extended buffer after train (20 units) so player can see obstacles after jumping down
                    let train_end = train_z + train_length / 2.0 + 20.0;
                    if z_pos >= ramp_start && z_pos <= train_end {
                        return true;
                    }
                }
            }
            false
        };

        // Only spawn if there's room and we need more obstacles ahead
        let spawn_threshold = player_z + min_spawn_distance;

        if max_barricade_z < spawn_threshold {
            // Spawn new obstacle well ahead of player
            let target_z = player_z + min_spawn_distance + 15.0;

            // Use obstacle count + player position as seed for better randomness
            let seed = pseudo_random(
                obstacle_count
                    .wrapping_add((target_z * 3.7) as u32)
                    .wrapping_add(0xDEADBEEF),
            );

            // Ensure variety in track selection
            let track_index = (seed % 3) as u8;

            // Skip if this would conflict with a ramped train
            if conflicts_with_ramp(track_index, target_z) {
                return;
            }

            // Mix obstacle types - more JumpOver obstacles (easier)
            let obstacle_type = if (seed / 7) % 4 == 0 {
                ObstacleType::SlideUnder
            } else {
                ObstacleType::JumpOver
            };

            spawn_obstacle(
                &mut commands,
                &mut meshes,
                &mut materials,
                track_index,
                target_z,
                obstacle_type,
                &game_config,
            );
        } else if max_barricade_z < player_z + max_spawn_distance {
            // Add more obstacles ahead but maintain spacing
            let next_z = max_barricade_z + min_obstacle_gap;

            if next_z < player_z + max_spawn_distance {
                // Different seed for additional obstacles
                let seed = pseudo_random(
                    obstacle_count
                        .wrapping_mul(31)
                        .wrapping_add((next_z * 5.3) as u32)
                        .wrapping_add(0xCAFEBABE),
                );
                let track_index = (seed % 3) as u8;

                // Skip if this would conflict with a ramped train
                if conflicts_with_ramp(track_index, next_z) {
                    return;
                }

                let obstacle_type = if (seed / 5) % 3 == 0 {
                    ObstacleType::SlideUnder
                } else {
                    ObstacleType::JumpOver
                };

                spawn_obstacle(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    track_index,
                    next_z,
                    obstacle_type,
                    &game_config,
                );
            }
        }
    }
}

pub fn scale_difficulty(mut game_config: ResMut<GameConfig>, time: Res<Time>) {
    game_config.difficulty_scale = 1.0 + (time.elapsed_secs() * 0.1).min(5.0);
}

pub fn recycle_barricades(
    mut commands: Commands,
    barricade_query: Query<(Entity, &Transform), With<Barricade>>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<Barricade>)>,
) {
    if let Ok(camera_transform) = camera_query.single() {
        let camera_z = camera_transform.translation.z;

        for (entity, transform) in barricade_query.iter() {
            if transform.translation.z < camera_z - 20.0 {
                commands.entity(entity).despawn();
            }
        }
    }
}
