use crate::camera::Camera;
use crate::gl;
use crate::gl::types::{GLfloat, GLint, GLuint};
use crate::init::*;
use matrix::{Matrix4x4, Vector3, Vector4};
use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct GlData {
    pub shader_programs: Vec<GLuint>,
    shader_program_indexes: HashMap<String, usize>,
    pub vertex_array_objects: Vec<GLuint>,
    pub textures: Vec<GLuint>,
    var_locations: Vec<HashMap<String, GLint>>,
    pub array_buffers: Vec<GLuint>,

    pub framebuffers: Vec<GLuint>,
    pub texture_attachments: Vec<GLuint>,
    pub render_buffer_attachments: Vec<GLuint>,

    pub uniform_buffers: Vec<GLuint>,
    uniform_buffers_indexes: HashMap<String, usize>,
}

impl GlData {
    pub fn new() -> GlData {
        let (shd_program_ids, shd_programs_indexes) = init_shader_programs();
        let (vertex_array_objects, array_buffers) = init_vertex_array_objects();
        let textures = init_textures(&shd_program_ids);
        let var_locations = get_variable_locations(&shd_program_ids);
        GlData {
            shader_programs: shd_program_ids,
            shader_program_indexes: shd_programs_indexes,
            vertex_array_objects,
            textures,
            var_locations,
            array_buffers,
            framebuffers: Vec::new(),
            texture_attachments: Vec::new(),
            render_buffer_attachments: Vec::new(),
            uniform_buffers: Vec::new(),
            uniform_buffers_indexes: HashMap::new(),
        }
    }

    pub fn get_var_loc(&self, name: &str, shader_program_index: usize) -> GLint {
        if let Some(loc) = self.var_locations[shader_program_index].get(name) {
            *loc
        } else {
            0
        }
    }

    #[allow(dead_code)]
    pub fn get_shader_program_gl_id(&self, key: &str) -> GLuint {
        if let Some(index) = self.shader_program_indexes.get(key) {
            self.shader_programs[*index]
        } else {
            panic!("There is no shader program OpenGL id for key: {}", key)
        }
    }

    pub fn get_shader_program_index(&self, key: &str) -> usize {
        if let Some(index) = self.shader_program_indexes.get(key) {
            *index
        } else {
            panic!("There is no shader program index for key: {}", key)
        }
    }

    #[allow(dead_code)]
    pub unsafe fn set_uniform_vec3f(
        &self,
        name: &str,
        shader_program_index: usize,
        vec: Vector3<f32>,
    ) {
        let var_location = self.get_var_loc(name, shader_program_index);
        let (v1, v2, v3) = vec.get_components();
        gl::Uniform3f(var_location, v1, v2, v3);
    }

    #[allow(dead_code)]
    pub unsafe fn set_uniform_vec4f(
        &self,
        name: &str,
        shader_program_index: usize,
        vec: Vector4<f32>,
    ) {
        let var_location = self.get_var_loc(name, shader_program_index);
        let (v1, v2, v3, v4) = vec.get_components();
        gl::Uniform4f(var_location, v1, v2, v3, v4);
    }

    pub unsafe fn set_uniform_3f(
        &self,
        name: &str,
        shader_program_index: usize,
        v1: f32,
        v2: f32,
        v3: f32,
    ) {
        let var_location = self.get_var_loc(name, shader_program_index);
        gl::Uniform3f(var_location, v1, v2, v3);
    }

    pub unsafe fn set_uniform_1f(&self, name: &str, shader_program_index: usize, v: f32) {
        let var_location = self.get_var_loc(name, shader_program_index);
        gl::Uniform1f(var_location, v);
    }

    pub unsafe fn set_uniform_mat4x4(
        &self,
        name: &str,
        shader_program_index: usize,
        matrix: &Matrix4x4<f32>,
    ) {
        let var_location = self.get_var_loc(name, shader_program_index);
        gl::UniformMatrix4fv(var_location, 1, gl::TRUE, matrix.as_ptr());
    }

    pub unsafe fn set_uniform_1u(&self, name: &str, shader_program_index: usize, v: u32) {
        let var_location = self.get_var_loc(name, shader_program_index);
        gl::Uniform1ui(var_location, v);
    }

    pub unsafe fn set_uniform_1i(&self, name: &str, shader_program_index: usize, v: i32) {
        let var_location = self.get_var_loc(name, shader_program_index);
        gl::Uniform1i(var_location, v);
    }

    pub fn get_uniform_buffer_gl_id(&self, key: &str) -> GLuint {
        if let Some(index) = self.uniform_buffers_indexes.get(key) {
            self.uniform_buffers[*index]
        } else {
            panic!("There is no uniform buffer OpenGL id for key: {}", key)
        }
    }

    pub fn insert_uniform_buffer(&mut self, ubo: GLuint, key: &str) {
        self.uniform_buffers_indexes
            .insert(key.to_string(), self.uniform_buffers.len());
        self.uniform_buffers.push(ubo);
    }
}

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
    pub specular_light_strength: f32,
    pub shininess: f32,*/
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
            specular_light_strength: 0.5,
            shininess: 32.0,*/
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
