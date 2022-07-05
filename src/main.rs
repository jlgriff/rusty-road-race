use rand::prelude::*;
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

    // Generate assets
    create_sprites(&mut game);

    // Create health text
    let health_message = game.add_text(ID_HEALTH_TEXT, "Health: 5");
    health_message.translation = Vec2::new(550.0, 320.0);

    // Game logic functions are run each frame
    // These will run in the order they're added here
    game.add_logic(player_movement_logic);
    game.add_logic(road_movement_logic);
    game.add_logic(collision_logic);

    // Run the game with an initial state
    game.run(GameState {
        health: 5,
        lost: false,
    });
}

fn create_sprites(game: &mut Game<GameState>) {
    // Create the road
    for i in 0..10 {
        let road_line = game.add_sprite(
            format!("{ID_ROAD_LINE_SPRITE}{}", i),
            SpritePreset::RacingBarrierWhite,
        );
        road_line.scale = 0.1;
        road_line.translation.x = -600.0 + 150.0 * i as f32;
    }

    // Create top barriers
    for i in 0..10 {
        let top_barrier = game.add_sprite(
            format!("{ID_BARRIER_SPRITE}_top_{}", i),
            SpritePreset::RacingBarrierWhite,
        );
        top_barrier.layer = 1.0;
        top_barrier.scale = SCALE_BARRIER;
        top_barrier.translation.x = WINDOW_MIN_X + 150.0 * i as f32;
        top_barrier.translation.y = WINDOW_MAX_Y;
        top_barrier.collision = true;
    }

    // Create bottom barriers
    for i in 0..10 {
        let bottom_barrier = game.add_sprite(
            format!("{ID_BARRIER_SPRITE}_bottom_{}", i),
            SpritePreset::RacingBarrierWhite,
        );
        bottom_barrier.layer = 1.0;
        bottom_barrier.scale = SCALE_BARRIER;
        bottom_barrier.translation.x = WINDOW_MIN_X + 150.0 * i as f32;
        bottom_barrier.translation.y = WINDOW_MIN_Y;
        bottom_barrier.collision = true;
    }

    // Create cars
    let car_presets = vec![
        SpritePreset::RacingCarBlack,
        SpritePreset::RacingCarGreen,
        SpritePreset::RacingCarRed,
        SpritePreset::RacingCarYellow,
    ];
    for (i, preset) in car_presets.into_iter().enumerate() {
        let car = game.add_sprite(format!("{ID_CAR_SPRITE}{}", i), preset);
        car.layer = 5.0;
        car.collision = true;
        car.translation.x =
            thread_rng().gen_range(WINDOW_MIN_OBSTACLE_GENERATION..WINDOW_MAX_OBSTACLE_GENERATION);
        car.translation.y = thread_rng().gen_range((WINDOW_MIN_Y + 60.0)..(WINDOW_MAX_Y - 60.0));
    }

    // Create obstacles
    let car_presets = vec![
        SpritePreset::RollingHoleStart,
        SpritePreset::RollingHoleEnd,
        SpritePreset::RollingHoleStart,
        SpritePreset::RollingHoleEnd,
    ];
    for (i, preset) in car_presets.into_iter().enumerate() {
        let obstacle = game.add_sprite(format!("{ID_OBSTACLE_SPRITE}{}", i), preset);
        obstacle.layer = 2.0;
        obstacle.collision = true;
        obstacle.translation.x =
            thread_rng().gen_range(WINDOW_MIN_OBSTACLE_GENERATION..WINDOW_MAX_OBSTACLE_GENERATION);
        obstacle.translation.y =
            thread_rng().gen_range((WINDOW_MIN_Y + 60.0)..(WINDOW_MAX_Y - 60.0));
    }
}

fn player_movement_logic(engine: &mut Engine, game_state: &mut GameState) {
    let mut direction = 0.0;
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Up, KeyCode::W])
    {
        direction += 1.0;
    }
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Down, KeyCode::S])
    {
        direction -= 1.0;
    }

    let player = engine.sprites.get_mut(ID_PLAYER_SPRITE).unwrap();
    player.translation.y += direction * SPEED_PLAYER * engine.delta_f32;
    player.rotation = direction * 0.15;
    if player.translation.y < WINDOW_MIN_Y || player.translation.y > WINDOW_MAX_Y {
        game_state.health = 0;
    }
}

fn road_movement_logic(engine: &mut Engine, _game_state: &mut GameState) {
    for sprite in engine.sprites.values_mut() {
        // Road and barrier movement
        if sprite.label.starts_with(ID_ROAD_LINE_SPRITE)
            || sprite.label.starts_with(ID_BARRIER_SPRITE)
        {
            sprite.translation.x -= SPEED_ROAD * engine.delta_f32;
            if sprite.translation.x < WINDOW_MIN_X {
                sprite.translation.x += WINDOW_MAX_X * 2.0;
            }
        }
        // Car movement
        if sprite.label.starts_with(ID_CAR_SPRITE) {
            sprite.translation.x -= SPEED_CARS * engine.delta_f32;
            if sprite.translation.x < WINDOW_MIN_X - 200.0 {
                sprite.translation.x = thread_rng()
                    .gen_range(WINDOW_MIN_OBSTACLE_GENERATION..WINDOW_MAX_OBSTACLE_GENERATION);
                sprite.translation.y =
                    thread_rng().gen_range((WINDOW_MIN_Y + 60.0)..(WINDOW_MAX_Y - 60.0));
            }
        }
        // Obstacle movement
        if sprite.label.starts_with(ID_OBSTACLE_SPRITE) {
            sprite.translation.x -= SPEED_ROAD * engine.delta_f32;
            if sprite.translation.x < WINDOW_MIN_X - 200.0 {
                sprite.translation.x = thread_rng()
                    .gen_range(WINDOW_MIN_OBSTACLE_GENERATION..WINDOW_MAX_OBSTACLE_GENERATION);
                sprite.translation.y =
                    thread_rng().gen_range((WINDOW_MIN_Y + 60.0)..(WINDOW_MAX_Y - 60.0));
            }
        }
    }
}

fn collision_logic(engine: &mut Engine, game_state: &mut GameState) {
    let health_message = engine.texts.get_mut(ID_HEALTH_TEXT).unwrap();
    for event in engine.collision_events.drain(..) {
        if !event.pair.either_contains(ID_PLAYER_SPRITE) || event.state.is_end() {
            continue;
        }
        if game_state.health > 0 {
            game_state.health -= 1;
            health_message.value = format!("Health: {}", game_state.health);
            engine.audio_manager.play_sfx(SfxPreset::Impact3, 1.0);
        }
    }
}

// ID constants
const ID_PLAYER_SPRITE: &str = "player";
const ID_ROAD_LINE_SPRITE: &str = "road_line";
const ID_BARRIER_SPRITE: &str = "barrier";
const ID_OBSTACLE_SPRITE: &str = "obstacle";
const ID_CAR_SPRITE: &str = "car";
const ID_HEALTH_TEXT: &str = "health";

// Graphical constants
const WINDOW_MIN_Y: f32 = -360.0;
const WINDOW_MAX_Y: f32 = 360.0;
const WINDOW_MIN_X: f32 = -675.0;
const WINDOW_MAX_X: f32 = 675.0;
const SCALE_BARRIER: f32 = 0.7;
const WINDOW_MIN_OBSTACLE_GENERATION: f32 = 700.0;
const WINDOW_MAX_OBSTACLE_GENERATION: f32 = 1800.0;

// Game constants
const VOLUME_MUSIC: f32 = 0.05;
const SPEED_PLAYER: f32 = 300.0;
const SPEED_ROAD: f32 = 900.0;
const SPEED_CARS: f32 = 250.0;
