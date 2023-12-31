use bevy::prelude::*;

pub mod components;
pub mod events;
pub mod resources;
mod systems;

use events::*;
use resources::*;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>() // add score resource with default value and keep tracking score not delete prev value
        .init_resource::<StarSpawnTimer>()
        .init_resource::<EnemySpawnTimer>()
        .init_resource::<HighScores>()
        .add_event::<GameOver>()
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_enemies)
        .add_startup_system(spawn_stars)
        .add_system(player_movement)
        .add_system(enemy_movement)
        .add_system(enemy_hit_player)
        .add_system(update_enemy_direction)
        .add_system(confine_player_movement)
        .add_system(cofine_enemy_movement)
        .add_system(player_hit_star)
        .add_system(update_score)
        .add_system(tick_star_spawn_timer)
        .add_system(spawn_star_overtime)
        .add_system(tick_enemy_spawn_timer)
        .add_system(spawn_enemies_overtime)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .add_system(update_high_scores)
        .add_system(high_scores_updated)
        .run();
}
