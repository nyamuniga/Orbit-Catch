mod render;
mod state;
mod systems;
mod utils;

use macroquad::audio::load_sound_from_bytes;
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
        let (lw, lh) = utils::logical_size();
        let camera = Camera2D {
            zoom: vec2(1.0 / (lw / 2.0), 1.0 / (lh / 2.0)),
            target: vec2(lw / 2.0, lh / 2.0),
            ..Default::default()
        };
        set_camera(&camera);

        // Keep sun centered dynamically
        game_state.sun.position = vec2(lw / 2.0, lh / 2.0);

        systems::update(&mut game_state, &chime);
        render::draw(&game_state);

        set_default_camera();
        next_frame().await
    }
}
