use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use game_state::GameContext;
use renderer::Renderer;
use sdl2::mixer::{InitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS, Music};

mod game_state;
mod renderer;
mod game_sound;

const GRID_X_SIZE: i32 = 20;
const GRID_Y_SIZE: i32 = 15;
const DOT_SIZE_IN_PXS: i32 = 20;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    // Inicializar SDL2 Mixer
    let frequency = 44_100; // Frecuencia de audio (44.1 kHz es común)
    let chunk_size = 1_024; // Tamaño del buffer de audio

    sdl2::mixer::open_audio(frequency, AUDIO_S16LSB, DEFAULT_CHANNELS, chunk_size)?;
    sdl2::mixer::init(InitFlag::OGG | InitFlag::MP3)?;
    sdl2::mixer::allocate_channels(4); // Número de canales de audio

    let sounds = game_sound::load_sounds()?;
    let music = Music::from_file("assets/summer-insects.mp3")?;
    Music::set_volume(32);
    music.play(-1)?; // Play music on a loop

    let window = video_subsystem
        .window(
            "snakisnaki",
            (GRID_X_SIZE + 2) as u32 * DOT_SIZE_IN_PXS as u32,
            (GRID_Y_SIZE + 2) as u32 * DOT_SIZE_IN_PXS as u32,
        )
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut context = GameContext::new();
    let font_path = "assets/Roboto-Regular.ttf";
    let mut font = ttf_context.load_font(font_path, 128)?;
    font.set_style(sdl2::ttf::FontStyle::NORMAL);
    let mut renderer = Renderer::new(window)?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut frame_counter = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {keycode: Some(keycode), ..} => {
                    match keycode {
                        Keycode::Up => context.move_up(),
                        Keycode::Down => context.move_down(),
                        Keycode::Left => context.move_left(),
                        Keycode::Right => context.move_right(),
                        Keycode::Space => context.toggle_pause(),
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        frame_counter += 1;
        if frame_counter % 5 == 0 {let state_before = context.state;
            let len_before = context.snake.len();
            context.update()?;
            if state_before != context.state {
                if let game_state::GameState::GameOver = context.state {
                    game_sound::play_sound(&sounds.game_over);
                }
            }
            if len_before < context.snake.len() {
                game_sound::play_sound(&sounds.eat_apple);
            }
            frame_counter = 0;
        }

        renderer.draw(&context, &font)?;
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}