use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct TrackPool {
    pub available_segments: Vec<Entity>,
    pub active_segments: Vec<Entity>,
    pub max_pool_size: usize,
}

impl TrackPool {
    pub fn new(max_pool_size: usize) -> Self {
        Self {
            available_segments: Vec::new(),
            active_segments: Vec::new(),
            max_pool_size,
        }
    }
}

