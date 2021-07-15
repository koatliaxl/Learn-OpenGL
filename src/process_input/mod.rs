mod camera;
mod lighting;
mod shadows;
mod textures;
mod view;

use crate::state_and_cfg::{Config, State};
use camera::*;
use glfw::{Action, CursorMode, Key, Window, WindowEvent};
use lighting::*;
use shadows::*;
use std::sync::mpsc::Receiver;
use textures::*;
use view::*;

pub fn process_input(
    window: &mut Window,
    events: &Receiver<(f64, WindowEvent)>,
    state: &mut State,
    config: &Config,
    delta_time: f32,
) {
    if window.get_key(Key::Escape) == Action::Press {
        window.set_should_close(true);
    }
    change_camera_pos(window, state, config, delta_time);
    change_view_params(window, state, config);

    for (_, event) in glfw::flush_messages(&events) {
        match event {
            WindowEvent::CursorEnter(b) => {
                if b {
                    window.set_cursor_mode(CursorMode::Disabled);
                } else {
                    window.set_cursor_mode(CursorMode::Normal);
                }
            }
            WindowEvent::CursorPos(cursor_x, cursor_y) => {
                let x_offset = cursor_x - state.last_cursor_pos.0;
                let y_offset = state.last_cursor_pos.1 - cursor_y;
                state.last_cursor_pos = (cursor_x, cursor_y);
                change_camera_angles(state, x_offset, y_offset);
            }
            WindowEvent::Scroll(_x_offset, y_offset) => {
                change_fov_by_scroll(state, y_offset);
            }
            WindowEvent::Key(key, _, action, _) => {
                if action == Action::Repeat || action == Action::Press {
                    match key {
                        Key::Up | Key::Down | Key::Left | Key::Right => {
                            change_textures(key,  state);
                        }
                        Key::Num1 | Key::Num2 | Key::Num3 | Key::Num4 | Key::Num5 | Key::Num6 |
                        //Key::Num7 | Key::Num8 | Key::Num9 | Key::Num0 |
                        Key::Minus | Key::Equal=> {
                            change_lighting(state, key);
                        }
                        Key::B => {
                            state.blinn_phong_lighting = !state.blinn_phong_lighting;
                            match state.blinn_phong_lighting {
                                true => println!("Blinn-Phong lighting"),
                                false => println!("Phong lighting"),
                            }
                        }
                        Key::C => {
                            use super::GammaCorrection::*;
                            match state.gamma_correction {
                                Disabled => {
                                    state.gamma_correction = BuiltinOpenGL;
                                    println!("Gamma Correction: OpenGL builtin");
                                }
                                BuiltinOpenGL => {
                                    state.gamma_correction = InShader;
                                    println!("Gamma Correction: In-shader");
                                }
                                InShader => {
                                    state.gamma_correction = Disabled;
                                    println!("Gamma Correction: disabled");
                                }
                            }
                        }
                        Key::X => {
                            state.srgb_texture = !state.srgb_texture;
                            match state.srgb_texture {
                                true => println!("Using sRGB(a) texture"),
                                false => println!("Using non-sRGB(a) texture"),
                            }
                        }
                        Key::Comma | Key::Period | Key::N | Key::M | Key::L => {
                            change_shadow_mapping_settings(state, key);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}
