use bevy::prelude::*;

pub const ENEMY_SPAWN_TIME: f32 = 5.0;
pub const STAR_SPAWN_TIME: f32 = 1.0;

// resource score
#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

//memberi default value pada score agar dapat digunakan pada init_resource pada fungsi main, maksudnya langsung specify jenis resource dengan value asli dan nilai tidak hilang ketika diubah
impl Default for Score {
    fn default() -> Score {
        Score { value: 0 }
    }
}

//uses to spawn stars overtime
#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}
//give default value
impl Default for StarSpawnTimer {
    fn default() -> StarSpawnTimer {
        StarSpawnTimer {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating), // set default value as repeat timer each one second
        }
    }
}

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}
//give default value
impl Default for EnemySpawnTimer {
    fn default() -> EnemySpawnTimer {
        EnemySpawnTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating), // set default value as repeat timer each enemy_spawn_time
        }
    }
}

#[derive(Resource, Debug)]
pub struct HighScores {
    pub scores: Vec<(String, u32)>,
}

impl Default for HighScores {
    fn default() -> HighScores {
        HighScores { scores: Vec::new() }
    }
}
