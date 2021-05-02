pub use InstancingOption::*;

use crate::gl;
use crate::gl::ARRAY_BUFFER;
use crate::state_and_cfg::{GlData, State};
use mat_vec::{Matrix4x4, Vector3};
use opengl_learn::{Model, SIZE_OF_GL_FLOAT};
use rand::{thread_rng, Rng};
use std::ffi::c_void;

#[derive(Copy, Clone, PartialEq)]
#[allow(unused)]
pub enum InstancingOption {
    Containers,
    Asteroids,
    AsteroidsNonInstanced,
}

static mut DRAW_OPTION: InstancingOption = Containers;
static mut ASTEROID_MODEL_MATRICES: Vec<Matrix4x4<f32>> = Vec::new();
static mut MODELS: Vec<Model> = Vec::new();
static mut ASTEROID_AMOUNT: u32 = 50000;

pub unsafe fn instancing_draw(gfx: &GlData) {
    if let Containers = DRAW_OPTION {
        gl::BindVertexArray(gfx.vertex_array_objects[1]);
        let arr_buf = gfx.get_array_buffer_gl_id("Offsets");
        gl::BindBuffer(ARRAY_BUFFER, arr_buf);
        let shd_idx = gfx.get_shader_program_index("Instancing shader");
        gl::UseProgram(gfx.shader_programs[shd_idx]);
        let model_mat = Matrix4x4::identity_matrix();
        gfx.set_uniform_mat4x4("model_mat", shd_idx, &model_mat);
        let tex_id = gfx.get_texture_gl_id("Container 2 texture");
        gl::BindTexture(gl::TEXTURE_2D, tex_id);
        gl::DrawElementsInstanced(
            gl::TRIANGLES, /* Rustfmt force vertical formatting */
            36,
            gl::UNSIGNED_INT,
            0 as *const c_void,
            100,
        );
    }
    if DRAW_OPTION == Asteroids || DRAW_OPTION == AsteroidsNonInstanced {
        // Draw planet
        let shd_idx = gfx.get_shader_program_index("UB Default shader");
        gl::UseProgram(gfx.shader_programs[shd_idx]);
        let planet_model_mat = Matrix4x4::new_scaling(3.0, 3.0, 3.0);
        gfx.set_uniform_mat4x4("model_mat", shd_idx, &planet_model_mat);
        MODELS[0].draw();
        // Draw asteroids
        let shd_idx = gfx.get_shader_program_index("Instancing shader");
        gl::UseProgram(gfx.shader_programs[shd_idx]);
        if let Asteroids = DRAW_OPTION {
            let ins_arr = gfx.get_array_buffer_gl_id("Asteroid Model Matrices");
            gl::BindBuffer(ARRAY_BUFFER, ins_arr);
            MODELS[1].instanced_draw(ASTEROID_AMOUNT as i32);
        }
        if let AsteroidsNonInstanced = DRAW_OPTION {
            for asteroid_model_mat in &ASTEROID_MODEL_MATRICES {
                gfx.set_uniform_mat4x4(
                    "model_mat",
                    shd_idx,
                    asteroid_model_mat, /* Rustfmt force vertical formatting */
                );
                MODELS[1].draw();
            }
        }
    }
}

pub unsafe fn setup_instancing(gfx: &mut GlData, opt: InstancingOption, state: &mut State) {
    DRAW_OPTION = opt;

    let shd_idx = gfx.get_shader_program_index("Instancing shader");
    gl::UseProgram(gfx.shader_programs[shd_idx]);
    gfx.set_uniform_1i("draw_option", shd_idx, DRAW_OPTION.int_code());

    if let Containers = DRAW_OPTION {
        let mut offsets = [0.0; 100 * 3];
        for i in 0..100 {
            offsets[i * 3] = 1.5 * (i % 10) as f32;
            offsets[i * 3 + 1] = 1.5 * (i / 10) as f32;
            //offsets[i * 3 + 2] = 0.0;
        }
        let mut instance_vbo = 0;
        gl::GenBuffers(1, &mut instance_vbo);
        gl::BindBuffer(ARRAY_BUFFER, instance_vbo);
        gl::BufferData(
            ARRAY_BUFFER,
            std::mem::size_of_val(&offsets) as isize,
            offsets.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );
        gfx.add_array_buffer(instance_vbo, "Offsets");

        gl::BindVertexArray(gfx.vertex_array_objects[1]);
        gl::EnableVertexAttribArray(8);
        gl::VertexAttribPointer(
            8,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * SIZE_OF_GL_FLOAT as i32,
            0 as *const c_void,
        );
        gl::VertexAttribDivisor(8, 1);

        gl::ActiveTexture(gl::TEXTURE0);
    }

    if DRAW_OPTION == Asteroids || DRAW_OPTION == AsteroidsNonInstanced {
        let mut planet_model = Model::new();
        planet_model.load_model("assets/planet/", "planet.obj");
        planet_model.load_textures_to_gl(
            gl::CLAMP_TO_BORDER,
            gl::CLAMP_TO_BORDER,
            gl::LINEAR_MIPMAP_LINEAR,
            gl::LINEAR,
        );
        planet_model.setup_draw();
        let mut asteroid_model = Model::new();
        asteroid_model.load_model("assets/rock/", "rock.obj");
        asteroid_model.load_textures_to_gl(
            gl::CLAMP_TO_BORDER,
            gl::CLAMP_TO_BORDER,
            gl::LINEAR_MIPMAP_LINEAR,
            gl::LINEAR,
        );
        asteroid_model.setup_draw();
        MODELS.push(planet_model);
        MODELS.push(asteroid_model);

        let radius = 30.0;
        let offset = 10.0;
        let mut d_rng = thread_rng();
        let mut displacement = move || d_rng.gen_range(-offset..=offset);
        let mut rng = thread_rng();
        for i in 0..ASTEROID_AMOUNT {
            // 1. Translation: displace along a circle with 'radius' in range [-offset; offset]
            let angle = i as f32 / ASTEROID_AMOUNT as f32 * 360.0;
            let x = angle.sin() * radius + displacement();
            let y = 0.2 * displacement();
            let z = angle.cos() * radius + displacement();
            let translation = Matrix4x4::new_translation(x, y, z);

            // 2. Scale: scale between 0.05 and 0.25
            let s = rng.gen_range(0.01..0.10);
            let scale = Matrix4x4::new_scaling(s, s, s);

            // 3. Rotation: add random rotation around a (semi)randomly picked rotation axis vector
            let rot_angle = rng.gen_range(0..360) as f32;
            let axis = Vector3::<f32>::new(0.4, 0.6, 0.8);
            let rotation = Matrix4x4::new_rotation(rot_angle, axis);

            let model = translation * rotation * scale;
            ASTEROID_MODEL_MATRICES.push(model);
        }

        if let Asteroids = DRAW_OPTION {
            let mut instancing_vbo = 0;
            gl::GenBuffers(1, &mut instancing_vbo);
            gl::BindBuffer(ARRAY_BUFFER, instancing_vbo);
            let mut raw_data = Vec::new();
            for matrix in &ASTEROID_MODEL_MATRICES {
                let matrix = matrix.transpose();
                let mat_raw = matrix.get_raw_data();
                for val in mat_raw {
                    raw_data.push(*val);
                }
            }
            gl::BufferData(
                ARRAY_BUFFER,
                raw_data.len() as isize * SIZE_OF_GL_FLOAT,
                raw_data.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );
            gfx.add_array_buffer(instancing_vbo, "Asteroid Model Matrices");

            const SIZE_OF_VEC4: i32 = (4 * SIZE_OF_GL_FLOAT) as i32;
            let vertex_arrays = MODELS[1].get_meshes_vertex_array_ids();
            for gl_id in vertex_arrays {
                gl::BindVertexArray(gl_id);
                gl::VertexAttribPointer(
                    4,
                    4,
                    gl::FLOAT,
                    gl::FALSE,
                    4 * SIZE_OF_VEC4,
                    0 as *const c_void,
                );
                gl::VertexAttribPointer(
                    5,
                    4,
                    gl::FLOAT,
                    gl::FALSE,
                    4 * SIZE_OF_VEC4,
                    (1 * SIZE_OF_VEC4) as *const c_void,
                );
                gl::VertexAttribPointer(
                    6,
                    4,
                    gl::FLOAT,
                    gl::FALSE,
                    4 * SIZE_OF_VEC4,
                    (2 * SIZE_OF_VEC4) as *const c_void,
                );
                gl::VertexAttribPointer(
                    7,
                    4,
                    gl::FLOAT,
                    gl::FALSE,
                    4 * SIZE_OF_VEC4,
                    (3 * SIZE_OF_VEC4) as *const c_void,
                );
                gl::EnableVertexAttribArray(4);
                gl::EnableVertexAttribArray(5);
                gl::EnableVertexAttribArray(6);
                gl::EnableVertexAttribArray(7);
                gl::VertexAttribDivisor(4, 1);
                gl::VertexAttribDivisor(5, 1);
                gl::VertexAttribDivisor(6, 1);
                gl::VertexAttribDivisor(7, 1);
            }
        }

        state.camera.speed = 30.0;
        state.camera.position = Vector3::new(0.0, 3.0, 20.0);
    }
}

impl InstancingOption {
    fn int_code(&self) -> i32 {
        match self {
            Containers => 3,
            Asteroids => 1,
            AsteroidsNonInstanced => 2,
        }
    }
}
