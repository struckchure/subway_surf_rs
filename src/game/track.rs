use crate::resources::track_pool::TrackPool;
use bevy::prelude::*;

#[derive(Component)]
pub struct TrackSegment {
    pub track_index: u8,
    pub segment_id: u32,
}

#[derive(Component)]
pub struct Track;

#[derive(Component)]
pub struct Rail;

#[derive(Component)]
pub struct Sleeper;

pub const SEGMENT_LENGTH: f32 = 40.0;
const RAIL_WIDTH: f32 = 0.1;
const RAIL_HEIGHT: f32 = 0.15;
const RAIL_SPACING: f32 = 0.4;
const SLEEPER_WIDTH: f32 = 1.2;
const SLEEPER_HEIGHT: f32 = 0.08;
const SLEEPER_DEPTH: f32 = 0.2;
const SLEEPER_SPACING: f32 = 2.0;

pub fn spawn_track_segment(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    track_index: u8,
    segment_id: u32,
    x_offset: f32,
    z_position: f32,
) -> Entity {
    // Rail mesh and material (metallic gray)
    let rail_mesh = meshes.add(Cuboid::new(RAIL_WIDTH, RAIL_HEIGHT, SEGMENT_LENGTH));
    let rail_material = materials.add(Color::srgb(0.4, 0.4, 0.45));

    // Sleeper mesh and material (wooden brown)
    let sleeper_mesh = meshes.add(Cuboid::new(SLEEPER_WIDTH, SLEEPER_HEIGHT, SLEEPER_DEPTH));
    let sleeper_material = materials.add(Color::srgb(0.35, 0.2, 0.1));

    let track_entity = commands
        .spawn((
            TrackSegment {
                track_index,
                segment_id,
            },
            Track,
            Transform::from_xyz(x_offset, 0.0, z_position),
            Visibility::default(),
        ))
        .with_children(|parent| {
            // Left rail
            parent.spawn((
                Rail,
                Mesh3d(rail_mesh.clone()),
                MeshMaterial3d(rail_material.clone()),
                Transform::from_xyz(-RAIL_SPACING, RAIL_HEIGHT / 2.0 + SLEEPER_HEIGHT, 0.0),
            ));

            // Right rail
            parent.spawn((
                Rail,
                Mesh3d(rail_mesh),
                MeshMaterial3d(rail_material),
                Transform::from_xyz(RAIL_SPACING, RAIL_HEIGHT / 2.0 + SLEEPER_HEIGHT, 0.0),
            ));

            // Sleepers
            let num_sleepers = (SEGMENT_LENGTH / SLEEPER_SPACING) as i32;
            let start_z = -SEGMENT_LENGTH / 2.0 + SLEEPER_SPACING / 2.0;

            for i in 0..num_sleepers {
                let z = start_z + (i as f32) * SLEEPER_SPACING;
                parent.spawn((
                    Sleeper,
                    Mesh3d(sleeper_mesh.clone()),
                    MeshMaterial3d(sleeper_material.clone()),
                    Transform::from_xyz(0.0, SLEEPER_HEIGHT / 2.0, z),
                ));
            }
        })
        .id();

    track_entity
}

pub fn generate_track_segments(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut track_pool: ResMut<TrackPool>,
    game_config: Res<crate::resources::game_config::GameConfig>,
) {
    let track_spacing = game_config.track_spacing;

    // Spawn multiple initial segments to ensure tracks are visible from the start
    // Start from behind the camera (z=-40) to ensure visibility
    for segment_offset in -1..6 {
        let z_position = (segment_offset as f32) * SEGMENT_LENGTH;
        
        for track_index in 0..3 {
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
}

pub fn recycle_track_segments(
    mut commands: Commands,
    mut track_pool: ResMut<TrackPool>,
    track_query: Query<(Entity, &Transform, &TrackSegment)>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<TrackSegment>)>,
) {
    if let Ok(camera_transform) = camera_query.single() {
        let camera_z = camera_transform.translation.z;

        for (entity, transform, _segment) in track_query.iter() {
            if transform.translation.z < camera_z - SEGMENT_LENGTH {
                if let Some(index) = track_pool.active_segments.iter().position(|&e| e == entity) {
                    track_pool.active_segments.remove(index);
                    track_pool.available_segments.push(entity);
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

pub fn extend_tracks_infinitely(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut track_pool: ResMut<TrackPool>,
    track_query: Query<&Transform, (With<TrackSegment>, Without<Camera3d>)>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<TrackSegment>)>,
    game_config: Res<crate::resources::game_config::GameConfig>,
) {
    if let Ok(camera_transform) = camera_query.single() {
        let camera_z = camera_transform.translation.z;
        let track_spacing = game_config.track_spacing;

        // Find the furthest track segment
        let mut max_z = camera_z;
        for transform in track_query.iter() {
            let z = transform.translation.z;
            // Only consider tracks that are reasonably close to the camera
            // This prevents using stale far-away track data after restart
            if z > max_z && z < camera_z + 500.0 {
                max_z = z;
            }
        }

        if max_z < camera_z + 100.0 {
            for track_index in 0..3 {
                let x_offset = (track_index as f32 - 1.0) * track_spacing;
                let z_position = max_z + SEGMENT_LENGTH;

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
    }
}
