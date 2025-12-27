use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub enum PropType {
    Building,
    Sign,
    Light,
}

#[derive(Component)]
pub struct Prop {
    pub prop_type: PropType,
}

pub fn generate_props(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game_config: Res<crate::resources::game_config::GameConfig>,
) {
    let track_spacing = game_config.track_spacing;
    let prop_types = [PropType::Building, PropType::Sign, PropType::Light];

    for i in 0..10 {
        let prop_type = prop_types[i % 3];
        let x_offset = if i % 2 == 0 {
            -track_spacing * 1.5
        } else {
            track_spacing * 1.5
        };
        let z_position = (i as f32) * 5.0;

        let (mesh, color) = match prop_type {
            PropType::Building => (
                meshes.add(Cuboid::new(1.0, 2.0, 1.0)),
                Color::srgb(0.5, 0.5, 0.6),
            ),
            PropType::Sign => (
                meshes.add(Cuboid::new(0.5, 1.0, 0.1)),
                Color::srgb(0.8, 0.2, 0.2),
            ),
            PropType::Light => (
                meshes.add(Cylinder::new(0.2, 1.5)),
                Color::srgb(0.9, 0.9, 0.3),
            ),
        };

        commands.spawn((
            Prop { prop_type },
            Mesh3d(mesh),
            MeshMaterial3d(materials.add(color)),
            Transform::from_xyz(x_offset, 1.0, z_position),
        ));
    }
}

pub fn recycle_props(
    mut commands: Commands,
    prop_query: Query<(Entity, &Transform), With<Prop>>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<Prop>)>,
) {
    if let Ok(camera_transform) = camera_query.single() {
        let camera_z = camera_transform.translation.z;

        for (entity, transform) in prop_query.iter() {
            if transform.translation.z < camera_z - 20.0 {
                commands.entity(entity).despawn();
            }
        }
    }
}
