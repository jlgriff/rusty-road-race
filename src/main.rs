use rusty_engine::prelude::*;

struct GameState {
    health: u8,
    lost: bool,
}

fn main() {
    // Create a game
    let mut game = Game::new();

    game.window_settings(WindowDescriptor {
        title: "Rusty Road Racing".to_string(),
        ..Default::default()
    });

    // Set up the game's initial parameters
    let player = game.add_sprite(ID_PLAYER_SPRITE, SpritePreset::RacingCarBlue);
    player.translation.x = -500.0;
    player.layer = 10.0;
    player.collision = true;

    // Play background music
    game.audio_manager
        .play_music(MusicPreset::Classy8Bit, VOLUME_MUSIC);

    // Create the road
    for i in 0..10 {
        let roadline = game.add_sprite(
            format!("{ID_ROAD_LINE_SPRITE}{}", i),
            SpritePreset::RacingBarrierWhite,
        );
        roadline.scale = 0.1;
        roadline.translation.x = -600.0 + 150.0 * i as f32;
    }

    // Game logic functions are run each frame
    // These will run in the order they're added here
    game.add_logic(player_movement_logic);
    game.add_logic(road_movement_logic);

    // Run the game with an initial state
    game.run(GameState {
        health: 5,
        lost: false,
    });
}

fn player_movement_logic(engine: &mut Engine, game_state: &mut GameState) {
    let mut direction = 0.0;
    if engine.keyboard_state.pressed(KeyCode::Up) {
        direction += 1.0;
    }
    if engine.keyboard_state.pressed(KeyCode::Down) {
        direction -= 1.0;
    }

    let player = engine.sprites.get_mut(ID_PLAYER_SPRITE).unwrap();
    player.translation.y += direction * SPEED_PLAYER * engine.delta_f32;
    player.rotation = direction * 0.15;
    if player.translation.y < WINDOW_MIN_Y || player.translation.y > WINDOW_MAX_Y {
        game_state.health = 0;
    }
}

fn road_movement_logic(engine: &mut Engine, game_state: &mut GameState) {
    for sprite in engine.sprites.values_mut() {
        if sprite.label.starts_with(ID_ROAD_LINE_SPRITE) {
            sprite.translation.x -= SPEED_ROAD_OBJECTS * engine.delta_f32;
            if sprite.translation.x < WINDOW_MIN_X {
                sprite.translation.x += WINDOW_MIN_X + WINDOW_MAX_X;
            }
        }
    }
}

// ID constants
const ID_PLAYER_SPRITE: &str = "player";
const ID_ROAD_LINE_SPRITE: &str = "road_line";

// Game constants
const WINDOW_MIN_Y: f32 = -360.0;
const WINDOW_MAX_Y: f32 = 360.0;
const WINDOW_MIN_X: f32 = -675.0;
const WINDOW_MAX_X: f32 = 675.0;
const VOLUME_MUSIC: f32 = 0.05;
const SPEED_PLAYER: f32 = 300.0;
const SPEED_ROAD_OBJECTS: f32 = 400.0;
