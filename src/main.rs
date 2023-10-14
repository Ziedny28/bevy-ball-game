use bevy::prelude::*;
use bevy::window::PrimaryWindow;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .run();
}

//entity player
#[derive(Component)]
pub struct Player {}

//system dengan parameter commands untuk memberikan command, window_query untuk mendapatkan width dan height pada window,asset_server untuk menggunakan asset
pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>, //resource yang akan digunakan untuk laod asset
) {
    let window = window_query.get_single().unwrap(); //mendapatkan referensi pada window

    //spawning entity in the midle of the screen, with bundle as parameter
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0), //set position to the middle of the screen
            texture: asset_server.load("sprites/ball_blue_large.png"), //load this asset
            ..default()
        },
        Player {},
    ));
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap(); //mendapatkan referensi pada window

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0), //set position to the middle of the screen
        ..default()
    });
}
