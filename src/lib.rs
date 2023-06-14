use rand::prelude::*;
use rusty_engine::prelude::Sprite;

pub mod game;
pub mod logic;

pub const PLAYER_LABEL: &str = "player";
pub const ROAD_LINE_LABEL: &str = "roadline";
pub const OBSTACLE_LABEL: &str = "obstacle";
pub const HEALTH_MSG_LABEL: &str = "health_message";
pub const GAME_OVER_LABEL: &str = "game_over";

pub const PLAYER_SPEED: f32 = 250.0;
pub const ROAD_SPEED: f32 = 400.0;

pub struct GameState {
    pub health: u8,
    pub lost: bool,
}

pub fn place_obstacle(obstacle: &mut Sprite) {
    obstacle.translation.x = thread_rng().gen_range(800.0..1600.0);
    obstacle.translation.y = thread_rng().gen_range(-300.0..300.0);
}
