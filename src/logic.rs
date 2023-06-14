use rusty_engine::audio::SfxPreset;
use rusty_engine::game::Engine;
use rusty_engine::keyboard::KeyCode;

use crate::{GameState, place_obstacle};
use crate::constants::{GAME_OVER_LABEL, HEALTH_MSG_LABEL, OBSTACLE_LABEL, PLAYER_LABEL, PLAYER_SPEED, ROAD_LINE_LABEL, ROAD_SPEED};

pub fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    if !game_state.lost {
        handle_player_movement(engine, game_state);
        handle_road_movement(engine);
        handle_collisions(engine, game_state);
        handle_game_over(engine, game_state);
    }
}

fn handle_game_over(engine: &mut Engine, game_state: &mut GameState) {
    if game_state.health == 0 {
        game_state.lost = true;
        let game_over = engine.add_text(GAME_OVER_LABEL, "GAME OVER");
        game_over.font_size = 128.0;
        engine.audio_manager.stop_music();
        engine.audio_manager.play_sfx(SfxPreset::Jingle3, 0.6);
    }
}

fn handle_collisions(engine: &mut Engine, game_state: &mut GameState) {
    let health_message = &mut engine.texts.get_mut(HEALTH_MSG_LABEL).unwrap();

    engine.collision_events.drain(..)
        .filter(|event| event.state.is_begin() || !event.pair.either_contains(PLAYER_LABEL))
        .for_each(|_| {
            if game_state.health > 0 {
                game_state.health -= 1;
                health_message.value = format!("Health: {}", game_state.health);
                engine.audio_manager.play_sfx(SfxPreset::Impact2, 0.6);
            }
        });
}

fn handle_road_movement(engine: &mut Engine) {
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

fn handle_player_movement(engine: &mut Engine, game_state: &mut GameState) {
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
    let player = engine.sprites.get_mut(PLAYER_LABEL).unwrap();
    player.translation.y += direction * PLAYER_SPEED * engine.delta_f32;
    player.rotation = direction * 0.15;

    // don't touch edges
    let player_y = player.translation.y;
    if -360.0 >= player_y || player_y >= 360.0 {
        game_state.health = 0;
        println!("left the road");
    }
}