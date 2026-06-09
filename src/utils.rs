use macroquad::prelude::*;

pub fn logical_size() -> (f32, f32) {
    let scale = screen_width().min(screen_height()) / 720.0;
    (screen_width() / scale, screen_height() / scale)
}

pub fn virtual_mouse() -> Vec2 {
    let scale = screen_width().min(screen_height()) / 720.0;
    let (x, y) = mouse_position();
    vec2(x / scale, y / scale)
}
