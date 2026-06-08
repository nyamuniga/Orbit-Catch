mod render;
mod state;
mod systems;

use macroquad::audio::{Sound, load_sound_from_bytes};
use macroquad::miniquad::conf::Icon;
use macroquad::prelude::*;
use state::GameState;

fn window_conf() -> Conf {
    let icon_img = image::load_from_memory(include_bytes!("../assets/icon.png")).unwrap();
    let small: [u8; 1024] = icon_img
        .resize_exact(16, 16, image::imageops::FilterType::Nearest)
        .to_rgba8()
        .into_raw()
        .try_into()
        .unwrap();
    let medium: [u8; 4096] = icon_img
        .resize_exact(32, 32, image::imageops::FilterType::Nearest)
        .to_rgba8()
        .into_raw()
        .try_into()
        .unwrap();
    let big: [u8; 16384] = icon_img
        .resize_exact(64, 64, image::imageops::FilterType::Nearest)
        .to_rgba8()
        .into_raw()
        .try_into()
        .unwrap();

    let icon = Icon { small, medium, big };

    Conf {
        window_title: "Orbit Catch".to_owned(),
        fullscreen: false,
        window_width: 1280,
        window_height: 720,
        high_dpi: true,
        icon: Some(icon),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_state = GameState::new();

    let chime = load_sound_from_bytes(include_bytes!("../assets/chime.wav"))
        .await
        .unwrap();

    loop {
        // Keep sun centered even if window resizes
        game_state.sun.position = vec2(screen_width() / 2.0, screen_height() / 2.0);

        systems::update(&mut game_state, &chime);
        render::draw(&game_state);

        next_frame().await
    }
}
