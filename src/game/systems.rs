use crate::game::barricade::{Barricade, CollisionShape, ObstacleType};
use crate::game::coin::Coin;
use crate::game::game_state::{GameOverText, GameState};
use crate::game::input::PlayerInput;
use crate::game::player::{
    AnimationState, CurrentTrack, LeftArm, LeftLeg, Player, RightArm, RightLeg, SlideTimer,
    Velocity, spawn_player,
};
use crate::game::props::generate_props;
use crate::game::track::{
    SEGMENT_LENGTH, TrackSegment, generate_track_segments, spawn_track_segment,
};
use crate::game::train::{Train, TrainPart, TrainRamp, TrainTop, TrainType};
use crate::resources::game_config::GameConfig;
use crate::resources::score::Score;
use crate::resources::track_pool::TrackPool;
use avian3d::prelude::*;
use bevy::prelude::*;

pub fn setup_tracks(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    track_pool: ResMut<crate::resources::track_pool::TrackPool>,
    game_config: Res<crate::resources::game_config::GameConfig>,
) {
    generate_track_segments(
        commands.reborrow(),
        meshes,
        materials,
        track_pool,
        game_config,
    );

    // Spawn ground plane for physics collision
    commands.spawn((
        RigidBody::Static,
        Collider::half_space(Vec3::Y),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

pub fn setup_props(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    game_config: Res<crate::resources::game_config::GameConfig>,
) {
    generate_props(commands, meshes, materials, game_config);
}

pub fn setup_player(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    game_config: Res<crate::resources::game_config::GameConfig>,
) {
    spawn_player(commands, meshes, materials, game_config);
}

pub fn move_player_forward(
    mut player_query: Query<&mut Transform, (With<Player>, Without<Camera3d>)>,
    velocity_query: Query<&Velocity, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(velocity) = velocity_query.single() {
        for mut transform in player_query.iter_mut() {
            transform.translation.z += velocity.forward * time.delta_secs();
        }
    }
}

pub fn accelerate_speed(
    mut game_config: ResMut<GameConfig>,
    mut velocity_query: Query<&mut Velocity, With<Player>>,
    time: Res<Time>,
) {
    game_config.current_speed += game_config.speed_acceleration_rate * time.delta_secs();

    for mut velocity in velocity_query.iter_mut() {
        velocity.forward = game_config.current_speed;
    }
}

pub fn handle_track_switching(
    mut player_query: Query<(&mut CurrentTrack, &mut Transform), (With<Player>, Without<Camera3d>)>,
    player_input: Res<PlayerInput>,
    animation_query: Query<&AnimationState, With<Player>>,
    game_config: Res<GameConfig>,
) {
    if let Ok(animation_state) = animation_query.single() {
        if *animation_state != AnimationState::Running {
            return;
        }
    } else {
        return;
    }

    if let Ok((mut current_track, mut transform)) = player_query.single_mut() {
        let track_spacing = game_config.track_spacing;

        if player_input.move_left {
            match *current_track {
                CurrentTrack::Left => {
                    *current_track = CurrentTrack::Middle;
                }
                CurrentTrack::Middle => {
                    *current_track = CurrentTrack::Right;
                }
                CurrentTrack::Right => {}
            }
        }

        if player_input.move_right {
            match *current_track {
                CurrentTrack::Left => {}
                CurrentTrack::Middle => {
                    *current_track = CurrentTrack::Left;
                }
                CurrentTrack::Right => {
                    *current_track = CurrentTrack::Middle;
                }
            }
        }

        // Instant snap to target track position
        let target_x = (current_track.as_index() as f32 - 1.0) * track_spacing;
        transform.translation.x = target_x;
    }
}

pub fn handle_slide_jump_input(
    mut commands: Commands,
    mut player_query: Query<
        (Entity, &mut AnimationState, &mut LinearVelocity, &Transform),
        With<Player>,
    >,
    player_input: Res<PlayerInput>,
) {
    if let Ok((entity, mut animation_state, mut linear_velocity, transform)) =
        player_query.single_mut()
    {
        // Check if grounded (Y position near ground, accounting for player size)
        let is_grounded = transform.translation.y < 1.6;

        if is_grounded && *animation_state == AnimationState::Jumping {
            *animation_state = AnimationState::Running;
        }

        if *animation_state == AnimationState::Running {
            if player_input.slide {
                *animation_state = AnimationState::Sliding;
                // Add slide timer (0.5 seconds)
                commands.entity(entity).insert(SlideTimer {
                    timer: Timer::from_seconds(0.5, TimerMode::Once),
                });
            } else if player_input.jump && is_grounded {
                *animation_state = AnimationState::Jumping;
                linear_velocity.y = 10.0; // Jump impulse
            }
        }
    }
}

pub fn handle_slide_timer(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut AnimationState, &mut SlideTimer), With<Player>>,
    time: Res<Time>,
) {
    for (entity, mut animation_state, mut slide_timer) in player_query.iter_mut() {
        slide_timer.timer.tick(time.delta());
        if slide_timer.timer.just_finished() {
            *animation_state = AnimationState::Running;
            commands.entity(entity).remove::<SlideTimer>();
        }
    }
}

pub fn handle_animations(mut player_query: Query<(&AnimationState, &mut Transform), With<Player>>) {
    for (animation_state, mut transform) in player_query.iter_mut() {
        match *animation_state {
            AnimationState::Sliding => {
                transform.scale.y = 0.5;
            }
            AnimationState::Running | AnimationState::Jumping => {
                transform.scale.y = 1.0;
            }
        }
    }
}

pub fn animate_player_limbs(
    time: Res<Time>,
    player_query: Query<(&AnimationState, &Velocity), With<Player>>,
    mut left_arm_query: Query<
        &mut Transform,
        (
            With<LeftArm>,
            Without<RightArm>,
            Without<LeftLeg>,
            Without<RightLeg>,
        ),
    >,
    mut right_arm_query: Query<
        &mut Transform,
        (
            With<RightArm>,
            Without<LeftArm>,
            Without<LeftLeg>,
            Without<RightLeg>,
        ),
    >,
    mut left_leg_query: Query<
        &mut Transform,
        (
            With<LeftLeg>,
            Without<LeftArm>,
            Without<RightArm>,
            Without<RightLeg>,
        ),
    >,
    mut right_leg_query: Query<
        &mut Transform,
        (
            With<RightLeg>,
            Without<LeftArm>,
            Without<RightArm>,
            Without<LeftLeg>,
        ),
    >,
) {
    let Ok((animation_state, velocity)) = player_query.single() else {
        return;
    };

    let elapsed = time.elapsed_secs();
    // Scale animation speed based on player's forward velocity
    let run_speed = velocity.forward * 0.3; // Reduced from 0.8 for smoother animation
    let swing_amount = 0.5;

    match *animation_state {
        AnimationState::Running => {
            // Running animation: arms and legs swing opposite to each other
            let swing = (elapsed * run_speed).sin() * swing_amount;

            // Left arm swings forward when right leg is forward
            if let Ok(mut transform) = left_arm_query.single_mut() {
                transform.rotation = Quat::from_rotation_x(swing);
            }

            // Right arm swings opposite to left arm
            if let Ok(mut transform) = right_arm_query.single_mut() {
                transform.rotation = Quat::from_rotation_x(-swing);
            }

            // Left leg swings opposite to left arm
            if let Ok(mut transform) = left_leg_query.single_mut() {
                transform.rotation = Quat::from_rotation_x(-swing);
            }

            // Right leg swings with left arm
            if let Ok(mut transform) = right_leg_query.single_mut() {
                transform.rotation = Quat::from_rotation_x(swing);
            }
        }
        AnimationState::Sliding => {
            // Sliding animation: arms forward, legs bent back
            if let Ok(mut transform) = left_arm_query.single_mut() {
                transform.rotation = Quat::from_rotation_x(-0.8);
            }
            if let Ok(mut transform) = right_arm_query.single_mut() {
                transform.rotation = Quat::from_rotation_x(-0.8);
            }
            if let Ok(mut transform) = left_leg_query.single_mut() {
                transform.rotation = Quat::from_rotation_x(1.2);
            }
            if let Ok(mut transform) = right_leg_query.single_mut() {
                transform.rotation = Quat::from_rotation_x(1.2);
            }
        }
        AnimationState::Jumping => {
            // Jumping animation: arms up, legs slightly tucked
            if let Ok(mut transform) = left_arm_query.single_mut() {
                transform.rotation = Quat::from_rotation_x(-0.5) * Quat::from_rotation_z(0.3);
            }
            if let Ok(mut transform) = right_arm_query.single_mut() {
                transform.rotation = Quat::from_rotation_x(-0.5) * Quat::from_rotation_z(-0.3);
            }
            if let Ok(mut transform) = left_leg_query.single_mut() {
                transform.rotation = Quat::from_rotation_x(0.3);
            }
            if let Ok(mut transform) = right_leg_query.single_mut() {
                transform.rotation = Quat::from_rotation_x(0.3);
            }
        }
    }
}

pub fn detect_collisions(
    mut next_state: ResMut<NextState<GameState>>,
    player_query: Query<(&Transform, &CurrentTrack, &AnimationState), With<Player>>,
    barricade_query: Query<(&Transform, &Barricade, &CollisionShape), Without<Player>>,
) {
    if let Ok((player_transform, player_track, animation_state)) = player_query.single() {
        let player_pos = player_transform.translation;
        let player_track_index = player_track.as_index();
        let player_y = player_pos.y;

        for (barricade_transform, barricade, collision_shape) in barricade_query.iter() {
            if barricade.track_index == player_track_index {
                let barricade_pos = barricade_transform.translation;

                // Check Z distance (are we at the obstacle?)
                let z_distance = (player_pos.z - barricade_pos.z).abs();
                if z_distance > 0.8 {
                    continue; // Not close enough yet
                }

                // Check if player avoided the obstacle based on position
                let hit = match barricade.obstacle_type {
                    ObstacleType::JumpOver => {
                        // Low obstacle (bar at y ~0.45, height ~0.7)
                        // Player must be high enough to clear it
                        let obstacle_top = 0.45 + collision_shape.size.y / 2.0;
                        let player_bottom = player_y - 0.8; // Player's feet
                        player_bottom < obstacle_top
                    }
                    ObstacleType::SlideUnder => {
                        // High obstacle (sign at y ~1.5, height ~0.6)
                        // Player can either slide UNDER or jump OVER
                        let obstacle_bottom = 1.2; // Bottom of the sign
                        let obstacle_top = 1.9; // Top of the sign (including lights)

                        let (player_top, player_bottom) =
                            if *animation_state == AnimationState::Sliding {
                                (player_y + 0.4, player_y) // Sliding - very low
                            } else {
                                (player_y + 1.0, player_y - 0.8) // Standing/jumping height
                            };

                        // Collision only if player overlaps with obstacle vertically
                        // No hit if player is completely above OR completely below
                        let player_above = player_bottom > obstacle_top;
                        let player_below = player_top < obstacle_bottom;

                        !(player_above || player_below) // Hit if NOT above AND NOT below
                    }
                };

                if hit {
                    next_state.set(GameState::GameOver);
                    return;
                }
            }
        }
    }
}

pub fn detect_train_collisions(
    mut next_state: ResMut<NextState<GameState>>,
    player_query: Query<(&Transform, &CurrentTrack), With<Player>>,
    train_query: Query<(&Transform, &Train), Without<Player>>,
) {
    if let Ok((player_transform, player_track)) = player_query.single() {
        let player_pos = player_transform.translation;
        let player_track_index = player_track.as_index();
        let player_y = player_pos.y;

        for (train_transform, train) in train_query.iter() {
            // Only check trains on the same track
            if train.track_index != player_track_index {
                continue;
            }

            let train_pos = train_transform.translation;
            let train_half_length = train.length / 2.0;

            // Check if player is within train's Z range
            let player_z = player_pos.z;
            let train_front = train_pos.z + train_half_length;
            let train_back = train_pos.z - train_half_length;

            // Train dimensions
            let train_height = 2.2;
            let train_top = train_height + 0.1; // Top of train (matched to physics ~2.3)

            // Check if player is ON TOP of the train
            let player_bottom = player_y - 0.8;

            // Ramp-specific logic for StationaryWithRamp trains
            if train.train_type == TrainType::StationaryWithRamp {
                // Ramp parameters (must match train.rs)
                let ramp_length = 6.0;
                let ramp_start_z = train_back - ramp_length;
                let ramp_end_z = train_back;

                // Check if player is in the ramp zone (from ramp start to train back)
                // Extended tolerance for smooth transitions
                let on_ramp_zone = player_z >= ramp_start_z - 2.0 && player_z <= ramp_end_z + 2.0;

                // If in ramp zone, allow the player (physics handles the collision)
                if on_ramp_zone {
                    // Calculate expected height on ramp for validation
                    let ramp_progress =
                        ((player_z - ramp_start_z) / (ramp_end_z - ramp_start_z)).clamp(0.0, 1.0);
                    let expected_ramp_y = ramp_progress * train_top;

                    // Allow if player is anywhere near the ramp height (generous tolerance)
                    // The physics colliders will handle the actual collision
                    if player_y >= -0.5 && player_bottom <= expected_ramp_y + 2.0 {
                        continue; // Player is on or near the ramp - let physics handle it
                    }
                }
            }

            // ALL trains: Check if player is on top of train OR exiting
            // Extended zone to allow safe exit from front/back of train
            let on_or_near_train = player_z >= train_back - 2.0 && player_z <= train_front + 3.0;
            if on_or_near_train {
                // If player is at train-top height or above, they're safe (on top of train)
                if player_bottom >= train_top - 1.5 {
                    continue; // Player is on top of train or safely exiting
                }
            }

            // Skip collision if player is elevated (jumping onto train)
            if player_y > 1.5 {
                continue;
            }

            // Check if within Z bounds of train body
            if player_z < train_back - 0.5 || player_z > train_front + 0.5 {
                continue; // Not near the train body
            }

            // Check if player is colliding with train body (only if at ground level)
            if player_bottom < train_top && player_y < train_top {
                // Collision with train!
                next_state.set(GameState::GameOver);
                return;
            }
        }
    }
}

pub fn handle_game_over_restart(
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    game_state: Res<State<GameState>>,
    ui_query: Query<Entity, With<GameOverText>>,
    mut game_config: ResMut<GameConfig>,
    mut track_pool: ResMut<TrackPool>,
    mut score: ResMut<Score>,
    mut player_query: Query<
        (
            &mut Transform,
            &mut CurrentTrack,
            &mut AnimationState,
            &mut Velocity,
        ),
        With<Player>,
    >,
    barricade_query: Query<Entity, With<Barricade>>,
    coin_query: Query<Entity, With<Coin>>,
    train_related_query: Query<
        Entity,
        Or<(
            With<Train>,
            With<TrainRamp>,
            With<TrainTop>,
            With<TrainPart>,
        )>,
    >,
    track_query: Query<Entity, With<TrackSegment>>,
    mut camera_query: Query<
        &mut Transform,
        (With<Camera3d>, Without<Player>, Without<TrackSegment>),
    >,
) {
    if *game_state.get() == GameState::GameOver {
        if keyboard_input.just_pressed(KeyCode::Space)
            || keyboard_input.just_pressed(KeyCode::Enter)
        {
            for entity in ui_query.iter() {
                commands.entity(entity).despawn();
            }

            for entity in barricade_query.iter() {
                commands.entity(entity).despawn();
            }

            // Despawn all coins
            for entity in coin_query.iter() {
                commands.entity(entity).despawn();
            }

            // Reset score
            score.reset();

            // Despawn all train-related entities (trains, ramps, tops, parts)
            for entity in train_related_query.iter() {
                commands.entity(entity).despawn();
            }

            // Despawn all track segments
            for entity in track_query.iter() {
                commands.entity(entity).despawn();
            }

            // Reset track pool
            track_pool.active_segments.clear();
            track_pool.available_segments.clear();

            // Regenerate initial tracks (inline to avoid type issues)
            let track_spacing = game_config.track_spacing;
            for segment_offset in -1..6i32 {
                let z_position = (segment_offset as f32) * SEGMENT_LENGTH;

                for track_index in 0..3u8 {
                    let x_offset = (track_index as f32 - 1.0) * track_spacing;
                    let segment_id = track_pool.active_segments.len() as u32;

                    let track_entity = spawn_track_segment(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        track_index,
                        segment_id,
                        x_offset,
                        z_position,
                    );

                    track_pool.active_segments.push(track_entity);
                }
            }

            game_config.current_speed = game_config.base_speed;
            game_config.difficulty_scale = 1.0;

            // Reset camera position to starting position
            if let Ok(mut camera_transform) = camera_query.single_mut() {
                camera_transform.translation = Vec3::new(0.0, 5.0, -10.0);
                camera_transform.look_at(Vec3::new(0.0, 0.0, 10.0), Vec3::Y);
            }

            if let Ok((mut transform, mut track, mut animation, mut velocity)) =
                player_query.single_mut()
            {
                transform.translation = Vec3::new(0.0, 1.5, 0.0);
                *track = CurrentTrack::Middle;
                *animation = AnimationState::Running;
                velocity.forward = game_config.base_speed;
            }

            next_state.set(GameState::Playing);
        }
    }
}

pub fn show_game_over_on_state_change(mut commands: Commands, game_state: Res<State<GameState>>) {
    if game_state.is_changed() && *game_state.get() == GameState::GameOver {
        commands.spawn((
            Text::new("GAME OVER\nPress SPACE or ENTER to restart"),
            Transform::from_xyz(0.0, 0.0, 100.0),
            GameOverText,
        ));
    }
}

// UI Component for coin display
#[derive(Component)]
pub struct CoinUI;

pub fn setup_coin_ui(mut commands: Commands) {
    commands.spawn((
        CoinUI,
        Text::new("Coins: 0"),
        TextFont {
            font_size: 32.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.85, 0.0)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            right: Val::Px(20.0),
            ..default()
        },
    ));
}

pub fn update_coin_ui(score: Res<Score>, mut query: Query<&mut Text, With<CoinUI>>) {
    for mut text in query.iter_mut() {
        **text = format!("Coins: {}", score.coins);
    }
}
