mod camera;
mod draw;
mod init;
mod load_tex;
mod process_input;
mod shaders;
mod state_and_cfg;

pub use ::opengl_learn::gl;
pub use draw::DRAW;

use self::draw::*;
use self::init::*;
use self::process_input::process_input;
use self::state_and_cfg::*;
use glfw::Context;
use opengl_learn::Model;
use std::fs::File;
use std::io::{Read, Write};
use std::time::Instant;

const CONFIG_FILE_PATH: &str = "config.txt";
//const _NULL: *const i32 = std::ptr::null();

fn main() {
    let (mut glfw, mut window, events) = init_glfw();
    init_open_gl(&mut window);
    let mut cfg = Config::new();
    config_file_exist_or_create(&cfg);
    read_config_file(&mut cfg);
    let mut state = State::new(window.get_size(), &cfg);
    let mut gfx = GlData::new();
    init_shader_programs(&mut gfx);
    init_textures(&mut gfx);
    #[allow(deprecated)]
    get_variable_locations(&mut gfx);
    get_variable_locations_2(&mut gfx);
    let mut model = Model::new();
    init_draw(&mut gfx, &mut model, &window, &mut state);

    let mut last_frame_time = Instant::now();

    while !window.should_close() {
        let delta_time = last_frame_time.elapsed().as_secs_f32();
        last_frame_time = Instant::now();
        let time = glfw.get_time();

        window.swap_buffers();
        glfw.poll_events();
        process_input(&mut window, &events, &mut state, &cfg, delta_time);
        draw(&gfx, &mut state, time as f32, &mut model, &window);
    }
    unsafe {
        gfx.free_gl_resources();
        model.free_gl_resources();
    }
}

fn config_file_exist_or_create(cfg: &Config) {
    if let Err(err) = File::open(CONFIG_FILE_PATH) {
        if let Ok(mut file) = File::create(CONFIG_FILE_PATH) {
            println!("\nConfig file created");
            let default_settings: String =
                format!("initial_camera_speed = {}", cfg.initial_camera_speed);
            if let Err(err) = file.write_all(default_settings.as_bytes()) {
                eprintln!("Error while writing to config file ({})", err)
            }
        } else {
            eprintln!("Couldn't create a config file ({})", err)
        }
    }
}

fn read_config_file(cfg: &mut Config) {
    let open_file_result = File::open(CONFIG_FILE_PATH);
    if let Ok(mut file) = open_file_result {
        let mut buf: String = String::new();
        let read_file_result = file.read_to_string(&mut buf);
        if let Err(err) = read_file_result {
            eprintln!("Couldn't read a config file ({})", err)
        } else if let Ok(_) = read_file_result {
            let rows = buf.split('\n');
            for row in rows {
                let key_val_pair = row.split(" = ").collect::<Vec<&str>>();
                if key_val_pair.len() == 2 {
                    match key_val_pair[0] {
                        "initial_camera_speed" => {
                            if let Ok(val) = key_val_pair[1].parse::<f32>() {
                                cfg.initial_camera_speed = val;
                            } else {
                                eprintln!("Config: invalid key value")
                            }
                        }
                        _ => eprintln!("Config: unknown key"),
                    }
                } else {
                    eprintln!("Invalid config format")
                }
            }
        }
    } else if let Err(err) = open_file_result {
        eprintln!("Couldn't open a config file ({})", err)
    }
}
