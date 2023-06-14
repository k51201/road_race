use rand::prelude::*;
use rusty_engine::prelude::*;

const PLAYER_LABEL: &str = "player";
const ROAD_LINE_LABEL: &str = "roadline";
const OBSTACLE_LABEL: &str = "obstacle";

const PLAYER_SPEED: f32 = 250.0;
const ROAD_SPEED: f32 = 400.0;

struct GameState {
    health: u8,
    lost: bool,
}

fn main() {
    let mut game = Game::new();

    init_player(&mut game);
    init_road_lines(&mut game);
    init_obstacles(&mut game);

    game.audio_manager.play_music(MusicPreset::WhimsicalPopsicle, 0.2);

    game.add_logic(game_logic);
    game.run(GameState {
        health: 5,
        lost: false,
    });
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

fn place_obstacle(obstacle: &mut Sprite) {
    obstacle.translation.x = thread_rng().gen_range(800.0..1600.0);
    obstacle.translation.y = thread_rng().gen_range(-300.0..300.0);
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

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    let mut direction: f32 = 0.0; // 1 - going up, 0 - not moving, -1 - going down

    // setting direction by the key(s) pressed
    let keyboard_state = &engine.keyboard_state;
    if keyboard_state.pressed(KeyCode::Up) {
        direction += 1.0;
    }
    if keyboard_state.pressed(KeyCode::Down) {
        direction -= 1.0;
    }

    // going up and down
    let mut player = engine.sprites.get_mut(PLAYER_LABEL).unwrap();
    player.translation.y += direction * PLAYER_SPEED * engine.delta_f32;
    player.rotation = direction * 0.15;

    // don't touch edges
    let player_y = player.translation.y;
    if -360.0 >= player_y || player_y >= 360.0 {
        game_state.health = 0;
        println!("left the road");
    }

    // road movement
    for sprite in engine.sprites.values_mut() {
        // road lines
        if sprite.label.starts_with(ROAD_LINE_LABEL) {
            let x = &mut sprite.translation.x;
            if *x < -675.0 {
                *x += 1500.0;
            } else {
                *x -= ROAD_SPEED * engine.delta_f32;
            }
        }

        // obstacles
        if sprite.label.starts_with(OBSTACLE_LABEL) {
            let x = &mut sprite.translation.x;
            if *x < -800.0 {
                place_obstacle(sprite);
            } else {
                *x -= ROAD_SPEED * engine.delta_f32;
            }
        }
    }
}