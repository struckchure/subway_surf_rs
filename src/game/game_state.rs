use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
}

#[derive(Component)]
pub struct GameOverText;

pub fn show_game_over_ui(mut commands: Commands) {
    commands.spawn((
        Text::new("GAME OVER\nPress SPACE or ENTER to restart"),
        Transform::from_xyz(0.0, 0.0, 100.0),
        GameOverText,
    ));
}

pub fn hide_game_over_ui(mut commands: Commands, ui_query: Query<Entity, With<GameOverText>>) {
    for entity in ui_query.iter() {
        commands.entity(entity).despawn();
    }
}
