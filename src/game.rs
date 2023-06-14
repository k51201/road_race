use rusty_engine::prelude::*;

use crate::*;

pub fn run_game(game_logic: fn(&mut Engine, &mut GameState)) {
    let mut game = Game::new();

    init_game(&mut game);

    game.add_logic(game_logic);
    game.run(GameState {
        health: 5,
        lost: false,
    });
}

fn init_game(mut game: &mut Game<GameState>) {
    init_player(&mut game);
    init_road_lines(&mut game);
    init_obstacles(&mut game);

    let health_message = game.add_text(HEALTH_MSG_LABEL, "Health: 5");
    health_message.translation = Vec2::new(550.0, 320.0);

    game.audio_manager.play_music(MusicPreset::WhimsicalPopsicle, 0.2);
}

fn init_obstacles(game: &mut Game<GameState>) {
    let obstacle_presets = vec!(SpritePreset::RacingBarrelRed, SpritePreset::RacingBarrelBlue, SpritePreset::RacingConeStraight);
    for (i, preset) in obstacle_presets.into_iter().enumerate() {
        let label = format!("{}{}", OBSTACLE_LABEL, i);
        let obstacle = game.add_sprite(label, preset);
        obstacle.layer = 5.0;
        obstacle.collision = true;
        place_obstacle(obstacle);
    }
}

fn init_road_lines(game: &mut Game<GameState>) {
    for i in 0..10 {
        let label = format!("{}{}", ROAD_LINE_LABEL, i);
        let mut line_sprite = game.add_sprite(label, SpritePreset::RacingBarrierWhite);
        line_sprite.scale = 0.1;
        line_sprite.translation.x = -600.0 + 150.0 * i as f32;
    }
}

fn init_player(game: &mut Game<GameState>) {
    let player = game.add_sprite(PLAYER_LABEL, SpritePreset::RacingCarYellow);
    player.translation.x = -500.0;
    player.layer = 10.0;
    player.collision = true;
}
