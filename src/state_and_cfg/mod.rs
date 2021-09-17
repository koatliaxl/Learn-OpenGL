mod gl_data;
pub use gl_data::*;

use crate::camera::Camera;
use crate::draw::{
    Attenuation, GammaCorrection, LightProjectionMatrix, OmnidirectionalShadowMappingSetting,
    PostProcessingOption, ShadowMappingSettings,
};
use opengl_learn::gl::types::GLfloat;
use std::time::{Duration, Instant};

pub struct State {
    pub zoom: GLfloat,
    pub mix: GLfloat,
    pub time_since_last_press: Instant,
    pub field_of_view: f32,
    pub aspect_ratio: f32,
    pub camera: Camera,
    pub last_cursor_pos: (f64, f64),
    /*pub ambient_light_strength: f32,
    pub diffuse_light_strength: f32,
    pub specular_light_strength: f32,*/
    pub shininess: f32,
    pub blinn_phong_lighting: bool,
    pub gamma_correction: GammaCorrection,
    pub srgb_texture: bool,
    pub attenuation: Attenuation,
    pub shadow_settings: ShadowMappingSettings,
    pub point_shadow_settings: OmnidirectionalShadowMappingSetting,
    pub post_processing_option: PostProcessingOption,
    pub normal_mapping: bool,
    pub tangent_space_correction: bool,
}

impl State {
    pub fn new(window_size: (i32, i32)) -> State {
        State {
            zoom: 1.0,
            mix: 0.0,
            time_since_last_press: Instant::now(),
            field_of_view: 60.0,
            aspect_ratio: 1.0,
            camera: Camera::new(),
            last_cursor_pos: (window_size.0 as f64 / 2.0, window_size.1 as f64 / 2.0),
            /*ambient_light_strength: 0.1,
            diffuse_light_strength: 1.0,
            specular_light_strength: 0.5,*/
            shininess: 32.0,
            blinn_phong_lighting: true,
            gamma_correction: GammaCorrection::Disabled,
            srgb_texture: false,
            attenuation: Attenuation {
                constant_term: 1.0,
                linear_term: 0.0,
                quadratic_term: 0.0,
            },
            shadow_settings: ShadowMappingSettings {
                min_shadow_bias: ShadowMappingSettings::DEFAULT_MIN_SHADOW_BIAS_FOR_PERSPECTIVE,
                max_shadow_bias: ShadowMappingSettings::DEFAULT_MAX_SHADOW_BIAS_FOR_PERSPECTIVE,
                cull_front_faces: false,
                projection_matrix: LightProjectionMatrix::Perspective,
                projection_fov: 90.0,
            },
            point_shadow_settings: OmnidirectionalShadowMappingSetting {
                visualize_depth_buffer: false,
                pcf: true,
                bias: 0.13,
                pcf_disk_radius: 0.05,
                disk_based_on_view_distance: false,
            },
            post_processing_option: PostProcessingOption::GaussianBlur3x3,
            normal_mapping: true,
            tangent_space_correction: true,
        }
    }
}

pub struct Config {
    pub repeat_delay: Duration,
}

impl Config {
    pub fn new() -> Config {
        Config {
            repeat_delay: Duration::from_millis(30),
        }
    }
}
