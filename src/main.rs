use rusty_engine::prelude::*;

struct GameState {
    health: u8,
    lost: bool,
}

const PLAYER_LABEL: &'static str = "player";

fn main() {
    let mut game = Game::new();

    init_player(&mut game);

    game.audio_manager.play_music(MusicPreset::WhimsicalPopsicle, 0.2);

    game.add_logic(game_logic);
    game.run(GameState {
        health: 5,
        lost: false,
    });
}

fn init_player(game: &mut Game<GameState>) {
    let player = game.add_sprite(PLAYER_LABEL, SpritePreset::RacingCarYellow);
    player.translation.x = -500.0;
    player.layer = 10.0;
    player.collision = true;
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // game logic goes here
}