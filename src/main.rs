use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

// player's variable
pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0; //player sprite size

//enemy's variable
pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0; //enemy sprite size

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_enemies)
        .add_system(player_movement)
        .add_system(enemy_movement)
        .add_system(update_enemy_direction)
        .add_system(confine_player_movement)
        .add_system(cofine_enemy_movement)
        .run();
}

//entity player
#[derive(Component)]
pub struct Player {}

//entity enemy
#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2, //keeping track of enemy's direction
}

/*
system untuk spawn player dengan parameter
 - commands untuk memberikan command,
 - window_query untuk mendapatkan width dan height pada window,
 - resources asset_server untuk menggunakan asset
 */
pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>, //resource yang akan digunakan untuk laod asset
) {
    let window = window_query.get_single().unwrap(); //mendapatkan referensi pada window

    //spawning entity in the midle of the screen, with bundle as parameter
    //spawning, must be inside bundle, since we write more than one component
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0), //set of player position to the middle of the screen
            texture: asset_server.load("sprites/ball_blue_large.png"), //load this asset
            ..default()
        },
        Player {},
    ));
}

/*
 system untuk spawn enemy dengan parameter
 - commands untuk memberikan command,
 - window_query untuk mendapatkan width dan height pada window,
 - resources asset_server untuk menggunakan asset
*/
pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>, //resource yang akan digunakan untuk laod asset
) {
    let window = window_query.get_single().unwrap(); //mendapatkan referensi pada window

    //spawn enemy
    for _ in 0..NUMBER_OF_ENEMIES {
        //get random position
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        //spawning, must be inside bundle, since we write more than one component
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(), //give diraction random value and then normalize
            },
        ));
    }
}

/*
system untuk spawn camera dengan parameter
 - commands untuk memberikan command,
 - window_query untuk mendapatkan width dan height pada window
*/
pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap(); //mendapatkan referensi pada window

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0), //set position of camera to the middle of the screen
        ..default()
    });
}

/*
system untuk spawn player dengan parameter
- keyboard input dengan tipe Resource Input Keycode untuk mengambil input,
 - player_query, untuk mendapat player dengan cara mendapatkan transform yang memiliki player(transform mut karena kita akan modifikasi value variable yang ditunjuk),
 - dan time untuk normalisasi waktu nanti
*/
pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    //akan memberikan true jika reference player_query ada dan mutable, kemudian mendapatkan transform
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        //move towards direction depends on the input
        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        //normalize
        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        //move_player
        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

/*
 system untuk membuat player tetap ada dalam layar dengan parameter
 - player_query, untuk mendapat player dengan cara mendapatkan transform yang memiliki player(transform mut karena kita akan modifikasi value variable yang ditunjuk),
 - window_query untuk mendapatkan width dan height pada window,
*/
pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    //akan memberikan true jika reference player_query ada dan mutable, kemudian mendapatkan transform
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap(); //mendapatkan referensi pada window

        let half_player_size: f32 = PLAYER_SIZE / 2.0;

        let x_min: f32 = 0.0 + half_player_size;
        let x_max: f32 = window.width() - half_player_size;
        let y_min: f32 = 0.0 + half_player_size;
        let y_max: f32 = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        //bound player's x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        //bound player's y position
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}

/*
 system untuk menggerakkan enemy, dengan parameter
 - enemy_query, untuk mendapat enemy dengan cara mendapatkan transform yang memiliki enemy(transform mut karena kita akan modifikasi value variable yang ditunjuk),
 - time, untuk gerakan independen
*/
pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    //loop on every enemy and get transform and enemy component
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0); //create vector 3 with random value for enemy's direction
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
        //move the enemy
    }
}

/* system untuk membuat enemy tetap ada dalam layar dengan parameter
 - enemy_query, untuk mendapat enemy dengan cara mendapatkan transform yang memiliki enemy,(enemy mut karena kita akan modifikasi value variable yang ditunjuk)
 - window_query untuk mendapatkan width dan height pada window,
 - audio, untuk efek suara
 - asset server untuk sprite
*/

pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap(); //mendapatkan referensi pada window

    let half_enemy_size = ENEMY_SIZE / 2.0; //mendapatkan ukuran setengah dari enemy

    //batas posisi
    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    //loop untuk tiap enemy
    for (transform, mut enemy) in enemy_query.iter_mut() {
        let mut direction_changed: bool = false; //untuk mendeteksi apakah arah berubah

        let translation = transform.translation; //current position

        //jika posisi diluar batas, kalikan direction dengan -1 untuk memberi efek memantul
        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }
        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }

        if direction_changed {
            //mengambil beberapa audio asset
            let sound_effect_1 = asset_server.load("audio/pluck_001.ogg");
            let sound_effect_2 = asset_server.load("audio/pluck_002.ogg");

            //memilih sound effect secara random
            let sound_effect = if random::<f32>() > 0.5 {
                sound_effect_1
            } else {
                sound_effect_2
            };

            //memainkan audio
            audio.play(sound_effect);
        }
    }
}

/*
 system for fixing bug of enemy stuck in the corner, with param
 -enemy_query is  untuk mendapat enemy dengan cara mendapatkan transform yang memiliki enemy,(transform mut karena kita akan modifikasi value variable yang ditunjuk)
 - window_query to get screen size
*/
pub fn cofine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap(); //mendapatkan referensi pada window

    let half_enemy_size: f32 = ENEMY_SIZE / 2.0;

    let x_min: f32 = 0.0 + half_enemy_size;
    let x_max: f32 = window.width() - half_enemy_size;
    let y_min: f32 = 0.0 + half_enemy_size;
    let y_max: f32 = window.height() - half_enemy_size;

    for mut transform in enemy_query.iter_mut() {
        let mut translation = transform.translation; //current position

        //bound enemy's x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        //bound enemy's y position
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        transform.translation = translation;
    }
}
