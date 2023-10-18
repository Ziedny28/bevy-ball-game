use bevy::prelude::*;

//entity player
#[derive(Component)]
pub struct Player {}

//entity enemy
#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2, //keeping track of enemy's direction
}

//component star
#[derive(Component)]
pub struct Star {}
