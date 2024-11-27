use sdl2::mixer::Chunk;

pub struct GameSounds {
    pub eat_apple: Chunk,
    pub game_over: Chunk,
}

pub fn load_sounds() -> Result<GameSounds, String> {
    let eat_apple = Chunk::from_file("assets/apple-eating.mp3")?;
    let game_over = Chunk::from_file("assets/game-over.mp3")?;

    Ok(GameSounds { eat_apple, game_over})
}

pub fn play_sound(sound: &Chunk) {
    sdl2::mixer::Channel::all().play(sound, 0).unwrap(); // Reproducir el sonido una vez
}