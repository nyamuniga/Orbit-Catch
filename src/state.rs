use macroquad::prelude::*;
use std::fs;
use std::path::PathBuf;

#[derive(PartialEq)]
pub enum AppState {
    MainMenu,
    InGame,
    Paused,
    GameOver,
}

#[derive(Clone, PartialEq)]
pub enum MoonState {
    Flying,
    Orbiting { ring_index: usize, angle: f32 },
}

#[derive(Clone)]
pub struct Moon {
    pub position: Vec2,
    pub velocity: Vec2,
    pub radius: f32,
    pub state: MoonState,
    pub trail: Vec<Vec2>, // For visual trail
}

pub struct Sun {
    pub position: Vec2,
    pub radius: f32,
    pub pulse_cooldown: f32,
}

pub struct Pulse {
    pub center: Vec2,
    pub current_radius: f32,
    pub max_radius: f32,
    pub speed: f32,
}

pub struct Ring {
    pub radius: f32,
    pub speed_multiplier: f32,
}

pub struct GameState {
    pub app_state: AppState,
    pub sun: Sun,
    pub moons: Vec<Moon>,
    pub pulse: Option<Pulse>,
    pub rings: Vec<Ring>,
    pub score: u32,
    pub spawn_timer: f32,

    // New Settings
    pub high_score: u32,
    pub sound_volume: f32,
    pub difficulty_multiplier: f32,
}

impl GameState {
    pub fn new() -> Self {
        let mut state = Self {
            app_state: AppState::MainMenu,
            sun: Sun {
                position: vec2(screen_width() / 2.0, screen_height() / 2.0),
                radius: 30.0,
                pulse_cooldown: 0.0,
            },
            moons: Vec::new(),
            pulse: None,
            rings: vec![
                Ring {
                    radius: 120.0,
                    speed_multiplier: 1.0,
                },
                Ring {
                    radius: 200.0,
                    speed_multiplier: 1.5,
                },
                Ring {
                    radius: 280.0,
                    speed_multiplier: 2.0,
                },
            ],
            score: 0,
            spawn_timer: 0.0,

            high_score: 0,
            sound_volume: 1.0,
            difficulty_multiplier: 1.0,
        };

        state.load_high_score();
        state
    }

    pub fn reset(&mut self) {
        self.sun.pulse_cooldown = 0.0;
        self.moons.clear();
        self.pulse = None;
        self.score = 0;
        self.spawn_timer = 0.0;
        self.app_state = AppState::InGame;
    }

    fn save_file_path() -> Option<PathBuf> {
        dirs::data_dir().map(|mut path| {
            path.push("orbit_catch");
            fs::create_dir_all(&path).ok();
            path.push("save_data.txt");
            path
        })
    }

    pub fn load_high_score(&mut self) {
        if let Some(path) = Self::save_file_path() {
            if let Ok(contents) = fs::read_to_string(path) {
                if let Ok(score) = contents.trim().parse::<u32>() {
                    self.high_score = score;
                }
            }
        }
    }

    pub fn save_high_score(&mut self) {
        if self.score > self.high_score {
            self.high_score = self.score;
            if let Some(path) = Self::save_file_path() {
                fs::write(path, self.high_score.to_string()).ok();
            }
        }
    }
}
