use crate::state_and_cfg::{Config, State};
use glfw::{Action, Key, Window};
use std::time::Instant;

pub fn change_view_params(window: &mut Window, state: &mut State, config: &Config) {
    if state.time_since_last_press.elapsed() < config.repeat_delay {
        return;
    }
    if window.get_key(Key::F) == Action::Press {
        state.time_since_last_press = Instant::now();
        state.field_of_view -= 0.5;
        println!("FOV: {}", state.field_of_view);
    }
    if window.get_key(Key::G) == Action::Press {
        state.time_since_last_press = Instant::now();
        state.field_of_view += 0.5;
        println!("FOV: {}", state.field_of_view);
    }
    if window.get_key(Key::R) == Action::Press {
        state.time_since_last_press = Instant::now();
        state.aspect_ratio -= 0.01;
        println!("Aspect ratio: {}", state.aspect_ratio);
    }
    if window.get_key(Key::T) == Action::Press {
        state.time_since_last_press = Instant::now();
        state.aspect_ratio += 0.01;
        println!("Aspect ratio: {}", state.aspect_ratio);
    }
}

pub fn change_fov_by_scroll(state: &mut State, y_offset: f64) {
    let sensitivity = 5.0;
    state.field_of_view -= y_offset as f32 * sensitivity;
    if state.field_of_view < 1.0 {
        state.field_of_view = 1.0
    }
    if state.field_of_view >= 179.0 {
        state.field_of_view = 179.0
    }
    println!("FOV: {}", state.field_of_view);
}
