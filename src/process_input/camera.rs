use crate::state_and_cfg::{Config, State};
use glfw::{Action, Key, Window};
use matrix::Vector3;
use std::time::Instant;

pub fn change_camera_pos(window: &mut Window, state: &mut State, config: &Config, delta_time: f32) {
    let key = if state.time_since_last_press.elapsed() > config.repeat_delay {
        if window.get_key(Key::A) == Action::Press {
            Key::A
        } else if window.get_key(Key::D) == Action::Press {
            Key::D
        } else if window.get_key(Key::W) == Action::Press {
            Key::W
        } else if window.get_key(Key::S) == Action::Press {
            Key::S
        } else if window.get_key(Key::Q) == Action::Press {
            Key::Q
        } else if window.get_key(Key::E) == Action::Press {
            Key::E
        } else {
            return;
        }
    } else {
        return;
    };
    state.time_since_last_press = Instant::now();

    let camera_move_value = 2.5;
    let camera_position = &mut state.camera.position;
    let world_up_dir = state.camera.world_up_direction;
    let camera_direction = state.camera.direction;

    let camera_pos_abs_change = match key {
        Key::A | Key::D => !(camera_direction ^ world_up_dir) * camera_move_value * delta_time,
        Key::W | Key::S => camera_direction * camera_move_value * delta_time,
        Key::Q | Key::E => {
            !(camera_direction ^ !(world_up_dir ^ camera_direction))
                * camera_move_value
                * delta_time
        }
        _ => unreachable!(),
    };
    match key {
        Key::A => *camera_position -= camera_pos_abs_change,
        Key::D => *camera_position += camera_pos_abs_change,
        Key::W => *camera_position += camera_pos_abs_change,
        Key::S => *camera_position -= camera_pos_abs_change,
        Key::Q => *camera_position -= camera_pos_abs_change,
        Key::E => *camera_position += camera_pos_abs_change,
        _ => unreachable!(),
    }
    //camera_position.set_y(0.0);
    //println!("Camera position: {:?}", camera_position.get_components());
}

pub fn change_camera_angles(state: &mut State, x_offset: f64, y_offset: f64) {
    let sensitivity = 0.4;
    state.camera.yaw += (x_offset * sensitivity) as f32;
    state.camera.pitch += (y_offset * sensitivity) as f32;
    if state.camera.pitch > 89.0 {
        state.camera.pitch = 89.0
    }
    if state.camera.pitch < -89.0 {
        state.camera.pitch = -89.0
    }
    //println!("yaw: {}, pitch: {}", state.camera.yaw, state.camera.pitch);
    let (yaw, pitch) = (
        state.camera.yaw.to_radians(),
        state.camera.pitch.to_radians(),
    );
    let direction = Vector3::from_array([
        yaw.cos() * pitch.cos(),
        pitch.sin(),
        yaw.sin() * pitch.cos(),
    ]);
    state.camera.direction = direction.normalize();
}
