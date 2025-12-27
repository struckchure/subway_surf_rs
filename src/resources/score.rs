use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Score {
    pub coins: u32,
}

impl Score {
    pub fn reset(&mut self) {
        self.coins = 0;
    }
}
