use germterm::{
    color::{Color, ColorGradient, GradientStop, sample_gradient},
    crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    draw::{draw_octad, draw_text, draw_twoxel},
    engine::{Engine, end_frame, exit_cleanup, init, start_frame},
    input::poll_input,
    layer::{LayerIndex, create_layer},
    particle::{ParticleColor, ParticleEmitter, ParticleSpec, spawn_particles},
    rich_text::{Attributes, RichText},
};
use rand::{RngExt, rngs::ThreadRng};
use std::io;

const TERM_COLS: u16 = 40;
const TERM_ROWS: u16 = 20;

const UP: (i16, i16) = (0, -1);
const LEFT: (i16, i16) = (-1, 0);
const DOWN: (i16, i16) = (0, 1);
const RIGHT: (i16, i16) = (1, 0);

enum GameState {
    Playing,
    GameOver,
}

fn main() -> io::Result<()> {
    let mut engine: Engine = Engine::new(TERM_COLS, TERM_ROWS)
        .title("snake")
        .limit_fps(0);

    let layer_0 = create_layer(&mut engine, 0);
    let layer_1 = create_layer(&mut engine, 1);
    let layer_2 = create_layer(&mut engine, 2);

    let bg_decoration_color: Color = Color(0x0a1a0aff);
    let movement_speed: f32 = 20.0;
    let mut segments: Vec<(i16, i16)> = vec![(20, 22), (20, 21), (20, 20), (20, 19)];
    let mut apple_pos: (i16, i16) = random_pos();
    let mut last_direction: (i16, i16) = DOWN;
    let mut direction: (i16, i16) = DOWN;
    let mut move_timer: f32 = 0.0;
    let mut apple_char: char = matrix_char();
    let mut char_timer: f32 = 0.0;
    let snake_color_gradient: ColorGradient = ColorGradient::new(vec![
        GradientStop::new(0.0, Color(0x00ff41ff)),
        GradientStop::new(1.0, Color(0x003b00ff)),
    ]);
    let mut game_state: GameState = GameState::Playing;

    init(&mut engine)?;

    'game_loop: loop {
        for event in poll_input() {
            match event {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    ..
                }) => break 'game_loop,
                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                }) => break 'game_loop,
                Event::Key(KeyEvent {
                    code: KeyCode::Char('w') | KeyCode::Up,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    if last_direction != DOWN {
                        direction = UP;
                    }
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('a') | KeyCode::Left,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    if last_direction != RIGHT {
                        direction = LEFT
                    }
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('s') | KeyCode::Down,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    if last_direction != UP {
                        direction = DOWN
                    }
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('d') | KeyCode::Right,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    if last_direction != LEFT {
                        direction = RIGHT
                    }
                }
                Event::Key(KeyEvent {
                    kind: KeyEventKind::Press,
                    ..
                }) if matches!(game_state, GameState::GameOver) => break 'game_loop,
                _ => (),
            }
        }

        start_frame(&mut engine);
        char_timer += engine.delta_time;
        if char_timer >= 0.06 {
            char_timer = 0.0;
            apple_char = matrix_char();
        }
        if matches!(game_state, GameState::Playing) {
            move_timer += engine.delta_time;
            let step_time: f32 = 1.0 / movement_speed;

            if move_timer >= step_time {
                move_timer -= step_time;
                last_direction = direction;

                let head: (i16, i16) = segments[0];
                let new_head = (
                    2 + (head.0 + direction.0 - 2).rem_euclid((TERM_COLS - 4) as i16),
                    2 + (head.1 + direction.1 - 2).rem_euclid((TERM_ROWS - 2) as i16 * 2),
                );

                if segments.contains(&new_head) {
                    game_state = GameState::GameOver;
                    spawn_death_explosion(
                        &mut engine,
                        layer_1,
                        new_head.0 as f32 + 0.5,
                        (new_head.1 as f32 + 0.5) * 0.5,
                    );
                }
                segments.insert(0, new_head);

                if new_head.0 == apple_pos.0 && new_head.1 / 2 == apple_pos.1 / 2 {
                    spawn_explosion(
                        &mut engine,
                        layer_0,
                        apple_pos.0 as f32 + 0.5,
                        (apple_pos.1 as f32 + 0.5) * 0.5,
                    );
                    apple_pos = random_pos();
                    spawn_apple_create_particles(
                        &mut engine,
                        layer_0,
                        (apple_pos.0 as f32) + 0.5,
                        ((apple_pos.1 as f32) + 0.5) * 0.5,
                    );
                } else {
                    segments.pop();
                }
            }
        }

        let mut draw = |x: f32, y: f32| {
            draw_octad(&mut engine, layer_2, x, y, bg_decoration_color);
        };

        // x axis borders
        for (dx, top, bottom, n) in [
            (1.5, 0.99, (TERM_ROWS - 1) as f32, TERM_COLS - 3),
            (1.0, 0.50, TERM_ROWS as f32 - 0.75, TERM_COLS - 2),
        ] {
            for x in 0..n {
                let xf = x as f32;
                draw(xf + dx, top);
                draw(xf + dx + 0.5, bottom);
            }
        }

        // y axis borders
        for (xl, xr, offl, offr, n) in [
            (1.99, (TERM_COLS - 2) as f32, 0.99, 1.0, TERM_ROWS * 2 - 3),
            (1.0, TERM_COLS as f32 - 1.5, 0.5, 0.75, TERM_ROWS * 2 - 2),
        ] {
            for y in 0..n {
                let yf = y as f32 * 0.5;
                draw(xl, yf + offl);
                draw(xr, yf + offr);
            }
        }

        // apple
        draw_text(
            &mut engine,
            layer_2,
            apple_pos.0 as i16,
            (apple_pos.1 / 2) as i16,
            RichText::new(apple_char.to_string().as_str())
                .with_fg(Color(0x00ff41ff))
                .with_attributes(Attributes::BOLD),
        );

        // snake
        for (i, segment) in segments.iter().enumerate() {
            let t: f32 = i as f32 / segments.len() as f32;
            // Multiplying the y axis by 0.5 here, as terminal cells usually have a 1:2 width to height ratio
            draw_twoxel(
                &mut engine,
                layer_2,
                segment.0 as f32,
                segment.1 as f32 * 0.5,
                sample_gradient(&snake_color_gradient, t),
            );
        }

        if matches!(game_state, GameState::GameOver) {
            draw_text(
                &mut engine,
                layer_2,
                (TERM_COLS / 2 - 6) as i16,
                (TERM_ROWS / 2 - 1) as i16,
                RichText::new("GAME OVER!")
                    .with_fg(Color(0x00ff41ff))
                    .with_attributes(Attributes::BOLD),
            );
            draw_text(
                &mut engine,
                layer_2,
                (TERM_COLS / 2 - 10) as i16,
                (TERM_ROWS / 2) as i16,
                RichText::new("press any key to exit")
                    .with_fg(Color(0x005c1eff)),
            );
        }

        end_frame(&mut engine)?;
    }

    exit_cleanup(&mut engine)?;
    Ok(())
}

fn matrix_char() -> char {
    // Offical matrix cmatrix rain character codes
    const RANGES: &[(u32, u32)] = &[
        (0x30A0, 0x30FF), // Katakana
        (0x2800, 0x28FF), // Braille
        (0x0021, 0x007E), // Printable ASCII
    ];
    let total: u32 = RANGES.iter().map(|(a, b)| b - a + 1).sum();
    let mut rng = rand::rng();
    let mut pick = rng.random_range(0..total);
    for &(start, end) in RANGES {
        let len = end - start + 1;
        if pick < len {
            return char::from_u32(start + pick).unwrap_or('?');
        }
        pick -= len;
    }
    '?'
}

fn random_pos() -> (i16, i16) {
    let mut rng: ThreadRng = rand::rng();
    (
        rng.random_range(2..(TERM_COLS - 2) as i16),
        rng.random_range(1..(TERM_ROWS - 1) as i16) * 2,
    )
}

fn spawn_explosion(engine: &mut Engine, layer: LayerIndex, x: f32, y: f32) {
    spawn_particles(
        engine,
        layer,
        x,
        y,
        &ParticleSpec {
            gravity_scale: 0.1,
            speed: 20.0..=70.0,
            lifetime_sec: 2.0,
            color: ParticleColor::Gradient(ColorGradient::new(vec![
                GradientStop::new(0.0, Color(0xccffccff)),
                GradientStop::new(0.1, Color(0x00ff41ff)),
                GradientStop::new(1.0, Color(0x003b0000)),
            ])),
        },
        &ParticleEmitter {
            count: 30,
            ..Default::default()
        },
    );
}

fn spawn_apple_create_particles(engine: &mut Engine, layer: LayerIndex, x: f32, y: f32) {
    spawn_particles(
        engine,
        layer,
        x,
        y,
        &ParticleSpec {
            gravity_scale: 0.0,
            speed: 8.0..=10.0,
            lifetime_sec: 0.7,
            color: ParticleColor::Gradient(ColorGradient::new(vec![
                GradientStop::new(0.0, Color(0x00ff4164)),
                GradientStop::new(1.0, Color(0x00ff4100)),
            ])),
        },
        &ParticleEmitter {
            count: 70,
            ..Default::default()
        },
    );
}

fn spawn_death_explosion(engine: &mut Engine, layer: LayerIndex, x: f32, y: f32) {
    spawn_particles(
        engine,
        layer,
        x,
        y,
        &ParticleSpec {
            gravity_scale: 0.5,
            speed: 10.0..=180.0,
            lifetime_sec: 2.5,
            color: ParticleColor::Gradient(ColorGradient::new(vec![
                GradientStop::new(0.0, Color(0xccffccff)),
                GradientStop::new(0.05, Color(0x00ff41ff)),
                GradientStop::new(0.4, Color(0x007a1eff)),
                GradientStop::new(1.0, Color(0x003b0000)),
            ])),
        },
        &ParticleEmitter {
            count: 500,
            ..Default::default()
        },
    );
}