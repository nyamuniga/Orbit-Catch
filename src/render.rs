use crate::state::{AppState, GameState, MoonState};
use macroquad::prelude::*;

pub fn draw(state: &GameState) {
    clear_background(Color::new(0.02, 0.02, 0.05, 1.0));

    match state.app_state {
        AppState::MainMenu => {
            let title = "ORBIT CATCH";
            let text_size = measure_text(title, None, 60, 1.0);
            draw_text(
                title,
                screen_width() / 2.0 - text_size.width / 2.0,
                screen_height() / 2.0 - 40.0,
                60.0,
                WHITE,
            );

            let prompt = "Tap or Click to Start";
            let prompt_size = measure_text(prompt, None, 30, 1.0);
            draw_text(
                prompt,
                screen_width() / 2.0 - prompt_size.width / 2.0,
                screen_height() / 2.0 + 40.0,
                30.0,
                GRAY,
            );

            let hs_text = format!("High Score: {}", state.high_score);
            let hs_size = measure_text(&hs_text, None, 20, 1.0);
            draw_text(
                &hs_text,
                screen_width() / 2.0 - hs_size.width / 2.0,
                screen_height() / 2.0 + 80.0,
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

            draw_circle(
                state.sun.position.x,
                state.sun.position.y,
                state.sun.radius,
                Color::new(1.0, 0.9, 0.4, 1.0),
            );
            draw_circle(
                state.sun.position.x,
                state.sun.position.y,
                state.sun.radius * 2.0,
                Color::new(1.0, 0.9, 0.4, 0.2),
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

            let score_text = format!("Score: {}", state.score);
            draw_text(&score_text, 20.0, 40.0, 40.0, WHITE);

            // Draw Pause button
            draw_rectangle(screen_width() - 50.0, 20.0, 10.0, 30.0, WHITE);
            draw_rectangle(screen_width() - 30.0, 20.0, 10.0, 30.0, WHITE);

            if state.app_state == AppState::Paused {
                draw_rectangle(
                    0.0,
                    0.0,
                    screen_width(),
                    screen_height(),
                    Color::new(0.0, 0.0, 0.0, 0.7),
                );

                let cx = screen_width() / 2.0;
                let cy = screen_height() / 2.0;

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

                draw_rectangle(
                    cx - 100.0,
                    cy + 100.0,
                    200.0,
                    50.0,
                    Color::new(0.2, 0.8, 0.4, 1.0),
                );
                let res_text = "RESUME";
                let res_size = measure_text(res_text, None, 30, 1.0);
                draw_text(res_text, cx - res_size.width / 2.0, cy + 135.0, 30.0, WHITE);
            }
        }
        AppState::GameOver => {
            let title = "GAME OVER";
            let text_size = measure_text(title, None, 60, 1.0);
            draw_text(
                title,
                screen_width() / 2.0 - text_size.width / 2.0,
                screen_height() / 2.0 - 40.0,
                60.0,
                RED,
            );

            let score_text = format!("Score: {}", state.score);
            let score_size = measure_text(&score_text, None, 30, 1.0);
            draw_text(
                &score_text,
                screen_width() / 2.0 - score_size.width / 2.0,
                screen_height() / 2.0 + 40.0,
                30.0,
                WHITE,
            );

            let hs_text = format!("High Score: {}", state.high_score);
            let hs_size = measure_text(&hs_text, None, 20, 1.0);
            draw_text(
                &hs_text,
                screen_width() / 2.0 - hs_size.width / 2.0,
                screen_height() / 2.0 + 70.0,
                20.0,
                GOLD,
            );

            let prompt = "Tap or Click to Restart";
            let prompt_size = measure_text(prompt, None, 20, 1.0);
            draw_text(
                prompt,
                screen_width() / 2.0 - prompt_size.width / 2.0,
                screen_height() / 2.0 + 110.0,
                20.0,
                GRAY,
            );
        }
    }
}
