use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct PlayerInput {
    pub move_left: bool,
    pub move_right: bool,
    pub jump: bool,
    pub slide: bool,
}

pub fn handle_keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_input: ResMut<PlayerInput>,
) {
    player_input.move_left = keyboard_input.just_pressed(KeyCode::ArrowLeft)
        || keyboard_input.just_pressed(KeyCode::KeyA);
    player_input.move_right = keyboard_input.just_pressed(KeyCode::ArrowRight)
        || keyboard_input.just_pressed(KeyCode::KeyD);
    player_input.jump = keyboard_input.just_pressed(KeyCode::Space);
    player_input.slide = keyboard_input.just_pressed(KeyCode::ArrowDown)
        || keyboard_input.just_pressed(KeyCode::KeyS);
}
