use rusty_engine::prelude::*;

use crate::*;

pub fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    if game_state.lost {
        return;
    }

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

    // game over
    if game_state.health == 0 {
        game_state.lost = true;
        let game_over = engine.add_text(GAME_OVER_LABEL, "GAME OVER");
        game_over.font_size = 128.0;
        engine.audio_manager.stop_music();
        engine.audio_manager.play_sfx(SfxPreset::Jingle3, 0.6);
    }
}