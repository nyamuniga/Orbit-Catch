use crate::state::{AppState, GameState, MoonState};
use crate::utils::logical_size;
use macroquad::prelude::*;

pub fn draw(state: &GameState) {
    let (lw, lh) = logical_size();
    clear_background(Color::new(0.02, 0.02, 0.05, 1.0));

    match state.app_state {
        AppState::MainMenu => {
            let title = "ORBIT CATCH";
            let text_size = measure_text(title, None, 60, 1.0);
            draw_text(
                title,
                lw / 2.0 - text_size.width / 2.0,
                lh / 2.0 - 40.0,
                60.0,
                WHITE,
            );

            let prompt = "Tap or Click to Start";
            let prompt_size = measure_text(prompt, None, 30, 1.0);
            draw_text(
                prompt,
                lw / 2.0 - prompt_size.width / 2.0,
                lh / 2.0 + 40.0,
                30.0,
                GRAY,
            );

            let hs_text = format!("High Score: {}", state.high_score);
            let hs_size = measure_text(&hs_text, None, 20, 1.0);
            draw_text(
                &hs_text,
                lw / 2.0 - hs_size.width / 2.0,
                lh / 2.0 + 80.0,
                20.0,
                GOLD,
            );
        }
        AppState::InGame | AppState::Paused => {
            for ring in &state.rings {
                draw_circle_lines(
                    state.sun.position.x,
                    state.sun.position.y,
                    ring.radius,
                    2.0,
                    Color::new(0.2, 0.2, 0.3, 0.5),
                );
            }

            for moon in &state.moons {
                let color = match moon.state {
                    MoonState::Flying => Color::new(1.0, 0.3, 0.3, 1.0),
                    MoonState::Orbiting { .. } => Color::new(0.3, 1.0, 0.8, 1.0),
                };

                if moon.trail.len() > 1 {
                    for i in 0..moon.trail.len() - 1 {
                        let alpha = i as f32 / moon.trail.len() as f32;
                        let mut trail_color = color;
                        trail_color.a = alpha * 0.5;
                        draw_line(
                            moon.trail[i].x,
                            moon.trail[i].y,
                            moon.trail[i + 1].x,
                            moon.trail[i + 1].y,
                            moon.radius,
                            trail_color,
                        );
                    }
                }

                draw_circle(moon.position.x, moon.position.y, moon.radius, color);
                draw_circle(
                    moon.position.x,
                    moon.position.y,
                    moon.radius * 2.0,
                    Color::new(color.r, color.g, color.b, 0.3),
                );
            }

            // Dynamic sun drawing
            let time = macroquad::time::get_time() as f32;
            let pulse_bonus = if state.sun.pulse_cooldown > 0.0 {
                (state.sun.pulse_cooldown / 0.5) * 10.0 // Scales up to +10 pixels
            } else {
                0.0
            };
            
            // Continuous pulsating effect based on time
            let continuous_pulse = (time * 3.0).sin() * 5.0; // +/- 5 pixels
            
            // 1. Draw smooth glow (many circles)
            let layers = 15;
            let max_glow_radius = state.sun.radius * 3.0 + pulse_bonus * 2.0 + continuous_pulse * 1.5;
            for i in 0..layers {
                let factor = i as f32 / layers as f32; // 0.0 to 1.0 (inner to outer)
                // Radius expands linearly
                let current_radius = state.sun.radius + (max_glow_radius - state.sun.radius) * factor;
                
                // Alpha decreases from core outward (exponential dropoff for smooth bloom)
                let alpha = 0.2 * (1.0 - factor).powi(2);
                
                draw_circle(
                    state.sun.position.x,
                    state.sun.position.y,
                    current_radius,
                    Color::new(1.0, 0.7 - factor * 0.3, 0.1, alpha),
                );
            }

            // 2. Draw core
            draw_circle(
                state.sun.position.x,
                state.sun.position.y,
                state.sun.radius + pulse_bonus * 0.5 + continuous_pulse * 0.5,
                Color::new(1.0, 0.9, 0.4, 1.0),
            );
            draw_circle(
                state.sun.position.x,
                state.sun.position.y,
                state.sun.radius * 1.2 + pulse_bonus * 0.5,
                Color::new(1.0, 0.8, 0.2, 0.5),
            );

            if let Some(pulse) = &state.pulse {
                draw_circle_lines(
                    pulse.center.x,
                    pulse.center.y,
                    pulse.current_radius,
                    4.0,
                    Color::new(0.4, 0.8, 1.0, 0.8),
                );
                draw_circle_lines(
                    pulse.center.x,
                    pulse.center.y,
                    pulse.current_radius,
                    12.0,
                    Color::new(0.4, 0.8, 1.0, 0.2),
                );
            }

            // Draw Mini-Radar
            if state.show_radar {
                let radar_radius = 70.0;
                let radar_center = vec2(lw - radar_radius - 30.0, lh - radar_radius - 30.0);
                let max_spawn_dist = lw.max(lh) * 0.7;
                let radar_scale = radar_radius / max_spawn_dist;
                
                // Army Radar Background & Border
                draw_circle(radar_center.x, radar_center.y, radar_radius, Color::new(0.0, 0.1, 0.0, 0.8));
                draw_circle_lines(radar_center.x, radar_center.y, radar_radius, 2.0, Color::new(0.2, 0.8, 0.2, 0.8));
                
                // Radar Crosshairs
                draw_line(radar_center.x - radar_radius, radar_center.y, radar_center.x + radar_radius, radar_center.y, 1.0, Color::new(0.1, 0.5, 0.1, 0.4));
                draw_line(radar_center.x, radar_center.y - radar_radius, radar_center.x, radar_center.y + radar_radius, 1.0, Color::new(0.1, 0.5, 0.1, 0.4));
                
                // Radar Sweeping Beam (Leading line + fading trail)
                let radar_time = macroquad::time::get_time() as f32;
                let sweep_angle = radar_time * 1.5;
                let sweep_end = radar_center + vec2(sweep_angle.cos(), sweep_angle.sin()) * radar_radius;
                let sweep_trail = radar_center + vec2((sweep_angle - 0.5).cos(), (sweep_angle - 0.5).sin()) * radar_radius;
                
                draw_triangle(radar_center, sweep_end, sweep_trail, Color::new(0.1, 0.8, 0.1, 0.25));
                draw_line(radar_center.x, radar_center.y, sweep_end.x, sweep_end.y, 2.0, Color::new(0.3, 1.0, 0.3, 0.8));
                
                // Radar Sun & Rings (Dim green)
                draw_circle(radar_center.x, radar_center.y, (state.sun.radius * radar_scale).max(2.0), Color::new(0.2, 0.6, 0.2, 0.8));
                for ring in &state.rings {
                    draw_circle_lines(radar_center.x, radar_center.y, ring.radius * radar_scale, 1.0, Color::new(0.1, 0.5, 0.1, 0.5));
                }
                
                // Radar Moons (Neon green blips)
                for moon in &state.moons {
                    let offset = moon.position - state.sun.position;
                    if offset.length() <= max_spawn_dist * 1.2 {
                        let radar_pos = radar_center + offset * radar_scale;
                        
                        let mut moon_angle = offset.y.atan2(offset.x);
                        if moon_angle < 0.0 { moon_angle += std::f32::consts::PI * 2.0; }
                        let normalized_sweep = sweep_angle % (std::f32::consts::PI * 2.0);
                        let mut age_angle = normalized_sweep - moon_angle;
                        if age_angle < 0.0 { age_angle += std::f32::consts::PI * 2.0; }
                        
                        // Alpha based on how recently the sweep passed it (fades out completely after 1.5 PI)
                        let fade = (1.0 - (age_angle / (std::f32::consts::PI * 1.5))).max(0.0);
                        
                        if fade > 0.0 {
                            let (mut dot_color, dot_size) = match moon.state {
                                MoonState::Flying => (Color::new(0.3, 1.0, 0.3, 1.0), 2.5),
                                MoonState::Orbiting { .. } => (Color::new(0.1, 0.6, 0.1, 0.8), 1.5),
                            };
                            dot_color.a *= fade; // Apply phosphor fade effect
                            draw_circle(radar_pos.x, radar_pos.y, dot_size, dot_color);
                        }
                    }
                }
            }

            let score_text = format!("Score: {}", state.score);
            draw_text(&score_text, 20.0, 40.0, 40.0, WHITE);

            // Draw Pause button
            draw_rectangle(lw - 50.0, 20.0, 10.0, 30.0, WHITE);
            draw_rectangle(lw - 30.0, 20.0, 10.0, 30.0, WHITE);

            if state.app_state == AppState::Paused {
                draw_rectangle(
                    0.0,
                    0.0,
                    lw,
                    lh,
                    Color::new(0.0, 0.0, 0.0, 0.7),
                );

                let cx = lw / 2.0;
                let cy = lh / 2.0;

                let title = "PAUSED";
                let title_size = measure_text(title, None, 50, 1.0);
                draw_text(title, cx - title_size.width / 2.0, cy - 100.0, 50.0, WHITE);

                draw_text("Volume", cx - 100.0, cy - 40.0, 20.0, GRAY);
                draw_rectangle(cx - 100.0, cy - 20.0, 200.0, 4.0, DARKGRAY);
                draw_rectangle(
                    cx - 100.0,
                    cy - 20.0,
                    200.0 * state.sound_volume,
                    4.0,
                    SKYBLUE,
                );
                draw_circle(
                    cx - 100.0 + 200.0 * state.sound_volume,
                    cy - 18.0,
                    10.0,
                    WHITE,
                );

                draw_text("Difficulty", cx - 120.0, cy + 30.0, 20.0, GRAY);

                let easy_col = if state.difficulty_multiplier == 0.5 {
                    SKYBLUE
                } else {
                    DARKGRAY
                };
                draw_rectangle(cx - 120.0, cy + 40.0, 60.0, 30.0, easy_col);
                draw_text("EASY", cx - 110.0, cy + 60.0, 20.0, WHITE);

                let norm_col = if state.difficulty_multiplier == 1.0 {
                    SKYBLUE
                } else {
                    DARKGRAY
                };
                draw_rectangle(cx - 30.0, cy + 40.0, 60.0, 30.0, norm_col);
                draw_text("NORM", cx - 25.0, cy + 60.0, 20.0, WHITE);

                let hard_col = if state.difficulty_multiplier == 2.0 {
                    SKYBLUE
                } else {
                    DARKGRAY
                };
                draw_rectangle(cx + 60.0, cy + 40.0, 60.0, 30.0, hard_col);
                draw_text("HARD", cx + 70.0, cy + 60.0, 20.0, WHITE);

                draw_text("Radar", cx - 120.0, cy + 100.0, 20.0, GRAY);
                let radar_col = if state.show_radar { SKYBLUE } else { DARKGRAY };
                draw_rectangle(cx - 30.0, cy + 90.0, 100.0, 30.0, radar_col);
                draw_text(if state.show_radar { "ON" } else { "OFF" }, cx - 5.0, cy + 110.0, 20.0, WHITE);

                draw_rectangle(
                    cx - 100.0,
                    cy + 150.0,
                    200.0,
                    50.0,
                    Color::new(0.2, 0.8, 0.4, 1.0),
                );
                let res_text = "RESUME";
                let res_size = measure_text(res_text, None, 30, 1.0);
                draw_text(res_text, cx - res_size.width / 2.0, cy + 185.0, 30.0, WHITE);
            }
        }
        AppState::GameOver => {
            let title = "GAME OVER";
            let text_size = measure_text(title, None, 60, 1.0);
            draw_text(
                title,
                lw / 2.0 - text_size.width / 2.0,
                lh / 2.0 - 40.0,
                60.0,
                RED,
            );

            let score_text = format!("Score: {}", state.score);
            let score_size = measure_text(&score_text, None, 30, 1.0);
            draw_text(
                &score_text,
                lw / 2.0 - score_size.width / 2.0,
                lh / 2.0 + 40.0,
                30.0,
                WHITE,
            );

            let hs_text = format!("High Score: {}", state.high_score);
            let hs_size = measure_text(&hs_text, None, 20, 1.0);
            draw_text(
                &hs_text,
                lw / 2.0 - hs_size.width / 2.0,
                lh / 2.0 + 70.0,
                20.0,
                GOLD,
            );

            let prompt = "Tap or Click to Restart";
            let prompt_size = measure_text(prompt, None, 20, 1.0);
            draw_text(
                prompt,
                lw / 2.0 - prompt_size.width / 2.0,
                lh / 2.0 + 110.0,
                20.0,
                GRAY,
            );
        }
    }
}
