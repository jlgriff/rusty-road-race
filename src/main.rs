use rusty_engine::prelude::*;

struct GameState {
    health: i32,
}

fn main() {
    // Create a game
    let mut game = Game::new();

    // Set up the game
    // `Game` exposes all of the methods and fields of `Engine`
    let sprite = game.add_sprite(ID_PLAYER_SPRITE, SpritePreset::RacingCarBlue);
    game.audio_manager
        .play_music(MusicPreset::Classy8Bit, VOLUME_MUSIC);

    // Add game logic functions, which will run in the order they're added here
    game.add_logic(game_logic);

    // Run the game, with an initial state
    let initial_game_state = GameState { health: 100 };
    game.run(initial_game_state);
}

// Run the game logic once each frame.
fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    let player = engine.sprites.get_mut(ID_PLAYER_SPRITE).unwrap();
}

// ID constants
const ID_PLAYER_SPRITE: &str = "player";

// Game constants
const VOLUME_MUSIC: f32 = 0.05;
