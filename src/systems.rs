use crate::state::{AppState, GameState, Moon, MoonState, Pulse};
use macroquad::audio::{PlaySoundParams, Sound, play_sound};
use macroquad::prelude::*;
use std::f32::consts::PI;

pub fn update(state: &mut GameState, chime: &Sound) {
    let dt = get_frame_time();

    let mouse_pos = mouse_position();
    let is_clicked = is_mouse_button_pressed(MouseButton::Left);
    let is_down = is_mouse_button_down(MouseButton::Left);

    match state.app_state {
        AppState::MainMenu => {
            if is_clicked || is_key_pressed(KeyCode::Space) {
                state.reset();
            }
        }
        AppState::InGame => {
            // Check Pause button click
            if is_clicked {
                if mouse_pos.0 >= screen_width() - 60.0 && mouse_pos.1 <= 60.0 {
                    state.app_state = AppState::Paused;
                    return;
                }
            }

            if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::P) {
                state.app_state = AppState::Paused;
                return;
            }

            if state.sun.pulse_cooldown > 0.0 {
                state.sun.pulse_cooldown -= dt;
            }

            let clicked_gameplay = (is_clicked
                && !(mouse_pos.0 >= screen_width() - 60.0 && mouse_pos.1 <= 60.0))
                || is_key_pressed(KeyCode::Space);

            if clicked_gameplay && state.sun.pulse_cooldown <= 0.0 {
                state.pulse = Some(Pulse {
                    center: state.sun.position,
                    current_radius: state.sun.radius,
                    max_radius: 600.0,
                    speed: 1000.0,
                });
                state.sun.pulse_cooldown = 0.5;
            }

            if let Some(pulse) = &mut state.pulse {
                pulse.current_radius += pulse.speed * dt;
                if pulse.current_radius >= pulse.max_radius {
                    state.pulse = None;
                }
            }

            state.spawn_timer -= dt;
            if state.spawn_timer <= 0.0 {
                spawn_moon(state);
                let base_spawn = 2.0 / (1.0 + state.score as f32 * 0.05);
                state.spawn_timer = base_spawn / state.difficulty_multiplier;
                state.spawn_timer = state.spawn_timer.max(0.3);
            }

            let mut new_moons = Vec::new();
            let mut game_over = false;

            for mut moon in state.moons.drain(..) {
                match moon.state {
                    MoonState::Flying => {
                        moon.position += moon.velocity * dt;

                        let dist_to_sun = moon.position.distance(state.sun.position);
                        if dist_to_sun < state.sun.radius + moon.radius {
                            game_over = true;
                        }

                        let mut caught = false;
                        if let Some(pulse) = &state.pulse {
                            let pulse_dist_diff = (dist_to_sun - pulse.current_radius).abs();
                            if pulse_dist_diff < 20.0 {
                                for (i, ring) in state.rings.iter().enumerate() {
                                    let ring_dist_diff = (dist_to_sun - ring.radius).abs();
                                    if ring_dist_diff < 25.0 {
                                        let angle = (moon.position.y - state.sun.position.y)
                                            .atan2(moon.position.x - state.sun.position.x);
                                        moon.state = MoonState::Orbiting {
                                            ring_index: i,
                                            angle,
                                        };
                                        caught = true;
                                        state.score += 1;

                                        play_sound(
                                            chime,
                                            PlaySoundParams {
                                                looped: false,
                                                volume: state.sound_volume,
                                            },
                                        );
                                        break;
                                    }
                                }

                                if !caught {
                                    let dir = (moon.position - state.sun.position).normalize();
                                    moon.velocity = dir * moon.velocity.length() * 1.5;
                                }
                            }
                        }

                        if dist_to_sun < 2000.0 {
                            new_moons.push(moon);
                        }
                    }
                    MoonState::Orbiting {
                        ring_index,
                        ref mut angle,
                    } => {
                        let ring = &state.rings[ring_index];
                        let angular_speed = (100.0 / ring.radius) * ring.speed_multiplier;
                        *angle += angular_speed * dt;
                        if *angle > PI * 2.0 {
                            *angle -= PI * 2.0;
                        }

                        moon.position =
                            state.sun.position + vec2(angle.cos(), angle.sin()) * ring.radius;

                        moon.trail.push(moon.position);
                        if moon.trail.len() > 20 {
                            moon.trail.remove(0);
                        }

                        new_moons.push(moon);
                    }
                }
            }
            state.moons = new_moons;

            for i in 0..state.moons.len() {
                for j in (i + 1)..state.moons.len() {
                    let m1 = &state.moons[i];
                    let m2 = &state.moons[j];
                    if m1.position.distance(m2.position) < m1.radius + m2.radius {
                        game_over = true;
                    }
                }
            }

            if game_over {
                state.save_high_score();
                state.app_state = AppState::GameOver;
            }
        }
        AppState::Paused => {
            if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::P) {
                state.app_state = AppState::InGame;
            }

            let cx = screen_width() / 2.0;
            let cy = screen_height() / 2.0;

            if is_clicked {
                if mouse_pos.0 > cx - 100.0
                    && mouse_pos.0 < cx + 100.0
                    && mouse_pos.1 > cy + 100.0
                    && mouse_pos.1 < cy + 150.0
                {
                    state.app_state = AppState::InGame;
                }
            }

            if is_down {
                if mouse_pos.0 >= cx - 110.0
                    && mouse_pos.0 <= cx + 110.0
                    && mouse_pos.1 > cy - 40.0
                    && mouse_pos.1 < cy + 10.0
                {
                    let mut new_vol = (mouse_pos.0 - (cx - 100.0)) / 200.0;
                    new_vol = new_vol.clamp(0.0, 1.0);
                    state.sound_volume = new_vol;
                }
            }

            if is_clicked {
                let y = cy + 40.0;
                if mouse_pos.1 > y && mouse_pos.1 < y + 30.0 {
                    if mouse_pos.0 > cx - 120.0 && mouse_pos.0 < cx - 60.0 {
                        state.difficulty_multiplier = 0.5;
                    } else if mouse_pos.0 > cx - 30.0 && mouse_pos.0 < cx + 30.0 {
                        state.difficulty_multiplier = 1.0;
                    } else if mouse_pos.0 > cx + 60.0 && mouse_pos.0 < cx + 120.0 {
                        state.difficulty_multiplier = 2.0;
                    }
                }
            }
        }
        AppState::GameOver => {
            if is_mouse_button_pressed(MouseButton::Left) || is_key_pressed(KeyCode::Space) {
                state.reset();
            }
        }
    }
}

fn spawn_moon(state: &mut GameState) {
    let angle = macroquad::rand::gen_range(0.0, PI * 2.0);
    let dist = screen_width().max(screen_height()) * 0.7;
    let pos = state.sun.position + vec2(angle.cos(), angle.sin()) * dist;

    let offset_angle: f32 = macroquad::rand::gen_range(-0.2, 0.2);
    let aim_dir = (state.sun.position - pos).normalize();
    let rotated_aim = vec2(
        aim_dir.x * offset_angle.cos() - aim_dir.y * offset_angle.sin(),
        aim_dir.x * offset_angle.sin() + aim_dir.y * offset_angle.cos(),
    );

    let base_speed = macroquad::rand::gen_range(100.0, 200.0) * (1.0 + state.score as f32 * 0.05);
    let speed = base_speed * state.difficulty_multiplier;

    state.moons.push(Moon {
        position: pos,
        velocity: rotated_aim * speed,
        radius: 8.0,
        state: MoonState::Flying,
        trail: Vec::new(),
    });
}
