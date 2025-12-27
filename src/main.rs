use avian3d::prelude::*;
use bevy::prelude::*;

mod game;
mod resources;

use game::camera::{follow_player, handle_viewport_resize, setup_camera};
use game::game_state::GameState;
use game::input::{PlayerInput, handle_keyboard_input};
use game::props::recycle_props;
use game::systems::{
    accelerate_speed, move_player_forward, setup_player, setup_props, setup_tracks,
};
use game::track::{extend_tracks_infinitely, recycle_track_segments};
use resources::game_config::GameConfig;
use resources::score::Score;
use resources::track_pool::TrackPool;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .init_state::<GameState>()
        .init_resource::<GameConfig>()
        .init_resource::<TrackPool>()
        .init_resource::<PlayerInput>()
        .init_resource::<Score>()
        .add_systems(
            Startup,
            (
                setup_camera,
                setup_tracks,
                setup_props,
                setup_player,
                game::systems::setup_coin_ui,
            ),
        )
        .add_systems(
            Update,
            (
                handle_keyboard_input,
                game::systems::show_game_over_on_state_change,
                game::systems::handle_game_over_restart,
                game::systems::update_coin_ui,
                handle_viewport_resize,
            ),
        )
        .add_systems(
            Update,
            (
                move_player_forward,
                accelerate_speed,
                game::systems::handle_track_switching,
                game::systems::handle_slide_jump_input,
                game::systems::handle_slide_timer,
                game::systems::handle_animations,
                game::systems::animate_player_limbs,
                game::systems::detect_collisions,
                game::systems::detect_train_collisions,
                game::barricade::generate_obstacles_procedurally,
                game::barricade::scale_difficulty,
                game::barricade::recycle_barricades,
            )
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            (
                game::train::generate_trains_procedurally,
                game::train::move_trains,
                game::train::recycle_trains,
                game::coin::generate_coins_procedurally,
                game::coin::collect_coins,
                game::coin::recycle_coins,
                extend_tracks_infinitely,
                recycle_track_segments,
                recycle_props,
                follow_player,
            )
                .run_if(in_state(GameState::Playing)),
        )
        .run();
}
