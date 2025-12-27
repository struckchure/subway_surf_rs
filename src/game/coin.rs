use crate::game::barricade::Barricade;
use crate::game::player::Player;
use crate::game::train::Train;
use crate::resources::game_config::GameConfig;
use crate::resources::score::Score;
use avian3d::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Coin;

// Simple pseudo-random for coin spawning
fn coin_random(seed: u32) -> u32 {
    let mut x = seed;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    x
}

pub fn spawn_coin(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    track_index: u8,
    z_position: f32,
    y_offset: f32,
    game_config: &GameConfig,
) -> Entity {
    let x_offset = (track_index as f32 - 1.0) * game_config.track_spacing;

    // Golden coin appearance
    let coin_color = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.85, 0.0),
        emissive: LinearRgba::new(0.8, 0.6, 0.0, 1.0),
        ..default()
    });

    let coin_mesh = meshes.add(Cylinder::new(0.25, 0.08));

    commands
        .spawn((
            Coin,
            Mesh3d(coin_mesh),
            MeshMaterial3d(coin_color),
            Transform::from_xyz(x_offset, 0.8 + y_offset, z_position)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
            // Sensor collider - detects overlap without blocking
            RigidBody::Static,
            Collider::cylinder(0.25, 0.08),
            Sensor,
        ))
        .id()
}

pub fn generate_coins_procedurally(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game_config: Res<GameConfig>,
    coin_query: Query<&Transform, With<Coin>>,
    player_query: Query<&Transform, (With<Player>, Without<Coin>)>,
    train_query: Query<(&Transform, &Train), (Without<Coin>, Without<Player>)>,
    barricade_query: Query<
        (&Transform, &Barricade),
        (Without<Coin>, Without<Player>, Without<Train>),
    >,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let player_z = player_transform.translation.z;

    // Don't spawn coins until player has moved forward a bit
    if player_z < 15.0 {
        return;
    }

    // Spawn parameters
    let min_spawn_distance = 40.0;
    let max_spawn_distance = 120.0;
    let min_coin_gap = 8.0;

    // Find the furthest coin
    let mut max_coin_z = player_z;
    let mut coin_count: u32 = 0;
    for transform in coin_query.iter() {
        coin_count += 1;
        if transform.translation.z > max_coin_z {
            max_coin_z = transform.translation.z;
        }
    }

    // Helper to check if position conflicts with obstacles
    let position_is_clear = |track_idx: u8, z_pos: f32| -> bool {
        let _track_x = (track_idx as f32 - 1.0) * game_config.track_spacing;

        // Check trains
        for (train_transform, train) in train_query.iter() {
            if train.track_index == track_idx {
                let train_z = train_transform.translation.z;
                let half_len = train.length / 2.0 + 2.0;
                if z_pos >= train_z - half_len && z_pos <= train_z + half_len {
                    return false;
                }
            }
        }

        // Check barricades
        for (barricade_transform, barricade) in barricade_query.iter() {
            if barricade.track_index == track_idx {
                let barricade_z = barricade_transform.translation.z;
                if (z_pos - barricade_z).abs() < 2.0 {
                    return false;
                }
            }
        }

        true
    };

    // Spawn coins ahead of player
    let spawn_threshold = player_z + min_spawn_distance;

    if max_coin_z < spawn_threshold || coin_count < 10 {
        let base_z = if max_coin_z > player_z {
            max_coin_z + min_coin_gap
        } else {
            player_z + min_spawn_distance
        };

        // Generate seed for randomness
        let seed = coin_random(
            coin_count
                .wrapping_add((base_z * 7.3) as u32)
                .wrapping_add(0xC01CA7E),
        );

        // Decide pattern type
        let pattern = seed % 4;

        match pattern {
            0 => {
                // Single line of coins on one track
                let track_index = ((seed / 4) % 3) as u8;
                for i in 0..5 {
                    let z_pos = base_z + (i as f32 * 2.0);
                    if z_pos < player_z + max_spawn_distance
                        && position_is_clear(track_index, z_pos)
                    {
                        spawn_coin(
                            &mut commands,
                            &mut meshes,
                            &mut materials,
                            track_index,
                            z_pos,
                            0.0,
                            &game_config,
                        );
                    }
                }
            }
            1 => {
                // Diagonal line across tracks
                let start_track = ((seed / 4) % 3) as i8;
                let direction: i8 = if (seed / 12) % 2 == 0 { 1 } else { -1 };
                for i in 0..5 {
                    let track = (start_track + direction * (i as i8 % 3)).rem_euclid(3) as u8;
                    let z_pos = base_z + (i as f32 * 2.5);
                    if z_pos < player_z + max_spawn_distance && position_is_clear(track, z_pos) {
                        spawn_coin(
                            &mut commands,
                            &mut meshes,
                            &mut materials,
                            track,
                            z_pos,
                            0.0,
                            &game_config,
                        );
                    }
                }
            }
            2 => {
                // Arc pattern (coins going up then down)
                let track_index = ((seed / 4) % 3) as u8;
                let heights = [0.0, 0.5, 1.0, 0.5, 0.0];
                for (i, &height) in heights.iter().enumerate() {
                    let z_pos = base_z + (i as f32 * 2.0);
                    if z_pos < player_z + max_spawn_distance
                        && position_is_clear(track_index, z_pos)
                    {
                        spawn_coin(
                            &mut commands,
                            &mut meshes,
                            &mut materials,
                            track_index,
                            z_pos,
                            height,
                            &game_config,
                        );
                    }
                }
            }
            _ => {
                // Cluster on all three tracks
                for track in 0..3u8 {
                    let z_pos = base_z + (track as f32 * 0.5);
                    if z_pos < player_z + max_spawn_distance && position_is_clear(track, z_pos) {
                        spawn_coin(
                            &mut commands,
                            &mut meshes,
                            &mut materials,
                            track,
                            z_pos,
                            0.0,
                            &game_config,
                        );
                    }
                }
            }
        }
    }
}

pub fn collect_coins(
    mut commands: Commands,
    mut score: ResMut<Score>,
    player_query: Query<&Transform, With<Player>>,
    coin_query: Query<(Entity, &Transform), With<Coin>>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let player_pos = player_transform.translation;

    for (coin_entity, coin_transform) in coin_query.iter() {
        let coin_pos = coin_transform.translation;

        // Check distance for collection (generous hitbox)
        let dx = (player_pos.x - coin_pos.x).abs();
        let dy = (player_pos.y - coin_pos.y).abs();
        let dz = (player_pos.z - coin_pos.z).abs();

        if dx < 1.0 && dy < 1.5 && dz < 1.0 {
            // Collect the coin
            score.coins += 1;
            commands.entity(coin_entity).despawn();
        }
    }
}

pub fn recycle_coins(
    mut commands: Commands,
    coin_query: Query<(Entity, &Transform), With<Coin>>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<Coin>)>,
) {
    let Ok(camera_transform) = camera_query.single() else {
        return;
    };

    let camera_z = camera_transform.translation.z;

    for (entity, transform) in coin_query.iter() {
        if transform.translation.z < camera_z - 15.0 {
            commands.entity(entity).despawn();
        }
    }
}
