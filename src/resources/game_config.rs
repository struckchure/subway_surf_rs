use bevy::prelude::*;

#[derive(Resource)]
pub struct GameConfig {
    pub base_speed: f32,
    pub speed_acceleration_rate: f32,
    pub current_speed: f32,
    pub barricade_spawn_base_interval: f32,
    pub difficulty_scale: f32,
    pub track_spacing: f32,
    pub barricade_advance_time: f32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            base_speed: 15.0,
            speed_acceleration_rate: 0.2,
            current_speed: 15.0,
            barricade_spawn_base_interval: 8.0, // More space between obstacles
            difficulty_scale: 1.0,
            track_spacing: 2.0,
            barricade_advance_time: 4.0, // Spawn further ahead
        }
    }
}
