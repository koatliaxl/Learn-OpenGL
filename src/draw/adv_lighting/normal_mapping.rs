use crate::draw::IDENTITY_MATRIX;
use crate::gl;
use crate::gl::types::GLuint;
use crate::init::{
    CUBE2_INDICES, CUBE2_RAW_VERTICES_NUM, CUBE2_VERTICES, CUBE_FACETS, INDICES_PER_CUBE2_FACET,
};
use crate::state_and_cfg::{GlData, State};
use mat_vec::{Matrix4x4, Vector3};
use opengl_learn::{SIZE_OF_GL_FLOAT, USIZE_OF_GL_FLOAT};
use std::ffi::c_void;

static mut CUBE_TANGENTS_AND_BITANGENTS_BUF: GLuint = 0;

pub unsafe fn draw_normal_mapping(gfx: &GlData, state: &State) {
    let shd_idx = match state.tangent_space_correction {
        true => 0,
        false => gfx.get_shader_program_index("Advanced Lighting shader"),
    };
    gl::UseProgram(gfx.shader_programs[shd_idx]);
    if !state.tangent_space_correction {
    } else {
    }
    gfx.set_uniform_vec3f("Viewer_Position", shd_idx, state.camera.position);
    gfx.set_uniform_1b("normal_mapping", shd_idx, state.normal_mapping);
    gfx.set_uniform_1f("Shininess", shd_idx, state.shininess);
    gl::DrawArrays(gl::TRIANGLES, 0, 36);

    let shd_id = gfx.get_shader_program_gl_id("Single Color shader");
    gl::UseProgram(shd_id);
    gl::DrawArrays(gl::TRIANGLES, 0, 36);
}

pub unsafe fn setup_normal_mapping(gfx: &mut GlData) {
    gl::BindVertexArray(gfx.vertex_array_objects[2]);
    gl::ActiveTexture(gl::TEXTURE0);
    gl::BindTexture(gl::TEXTURE_2D, gfx.get_texture_gl_id("Brick wall"));
    gl::ActiveTexture(gl::TEXTURE1);
    gl::BindTexture(gl::TEXTURE_2D, gfx.get_texture_gl_id("Brick wall normal"));

    let light_source_pos = Vector3::new(1.0, 1.0, 2.0);

    let shd_idx = gfx.get_shader_program_index("Advanced Lighting shader");
    gl::UseProgram(gfx.shader_programs[shd_idx]);
    gfx.set_uniform_1i("Light_Sources_Num", shd_idx, 1);
    gfx.set_uniform_mat4x4("model_mat", shd_idx, &IDENTITY_MATRIX);
    gfx.set_uniform_1i("normal_map", shd_idx, 1);
    gfx.set_uniform_vec3f("Light_Sources[0].position", shd_idx, light_source_pos);
    gfx.set_uniform_3f("Light_Sources[0].color", shd_idx, 1.0, 1.0, 1.0);
    gfx.set_uniform_1f("attenuation_linear_term", shd_idx, 0.1);
    gfx.set_uniform_1f("attenuation_quadratic_term", shd_idx, 0.03);
    gfx.set_uniform_1u("normal_mapping", shd_idx, 1);
    //gfx.set_uniform_1f("specular_factor", shd_idx, 0.4);

    let shd_idx = gfx.get_shader_program_index("Single Color shader");
    gl::UseProgram(gfx.shader_programs[shd_idx]);
    let scaling = Matrix4x4::new_uniform_scaling(0.1);
    //let (lx, ly, lz) = light_source_pos.get_components();
    let translation = Matrix4x4::new_translation_from_vec(light_source_pos);
    let model_mat = translation * scaling;
    gfx.set_uniform_mat4x4("model_mat", shd_idx, &model_mat);
    gfx.set_uniform_vec3f("color", shd_idx, Vector3::new(1.0, 1.0, 1.0));

    const VEC3_COMPONENTS: usize = 3;
    const NUM_OF_COMPONENTS: usize = VEC3_COMPONENTS * 2; // tangent + bitangent
    let mut tangents_and_bitangents = [0.0; CUBE2_RAW_VERTICES_NUM * NUM_OF_COMPONENTS];
    for facet in 0..CUBE_FACETS {
        for triangle in 0..2 {
            let (pos1, tex_coord_1) = CUBE2_VERTICES[CUBE2_INDICES[facet][0] as usize];
            let pos1 = Vector3::from_tuple(pos1);
            let tex_coord_1 = Vector3::new(tex_coord_1.0, tex_coord_1.1, 0.0);
            let (pos2, tex_coord_2) = CUBE2_VERTICES[CUBE2_INDICES[facet][0 + 1] as usize];
            let pos2 = Vector3::from_tuple(pos2);
            let tex_coord_2 = Vector3::new(tex_coord_2.0, tex_coord_2.1, 0.0);
            let (pos3, tex_coord_3) = CUBE2_VERTICES[CUBE2_INDICES[facet][0 + 2] as usize];
            let pos3 = Vector3::from_tuple(pos3);
            let tex_coord_3 = Vector3::new(tex_coord_3.0, tex_coord_3.1, 0.0);

            let edge_1 = pos1 - pos2;
            let edge_2 = pos1 - pos3;
            let delta_uv1 = tex_coord_1 - tex_coord_2;
            let delta_uv2 = tex_coord_1 - tex_coord_3;

            // reciprocal of determinant of delta texture coordinate matrix
            let rd = 1.0 / (delta_uv1.x() * delta_uv2.y() - delta_uv2.x() * delta_uv1.y());

            let tangent_x = rd * (delta_uv2.y() * edge_1.x() - delta_uv1.y() * edge_2.x());
            let tangent_y = rd * (delta_uv2.y() * edge_1.y() - delta_uv1.y() * edge_2.y());
            let tangent_z = rd * (delta_uv2.y() * edge_1.z() - delta_uv1.y() * edge_2.z());
            let bitangent_x = rd * (-delta_uv2.x() * edge_1.x() - delta_uv1.x() * edge_2.x());
            let bitangent_y = rd * (-delta_uv2.x() * edge_1.y() - delta_uv1.x() * edge_2.y());
            let bitangent_z = rd * (-delta_uv2.x() * edge_1.z() - delta_uv1.x() * edge_2.z());

            for vertex in (triangle * 3)..INDICES_PER_CUBE2_FACET / (2 - triangle) {
                tangents_and_bitangents[facet * 6 + vertex * NUM_OF_COMPONENTS + 0] = tangent_x;
                tangents_and_bitangents[facet * 6 + vertex * NUM_OF_COMPONENTS + 1] = tangent_y;
                tangents_and_bitangents[facet * 6 + vertex * NUM_OF_COMPONENTS + 2] = tangent_z;
                tangents_and_bitangents[facet * 6 + vertex * NUM_OF_COMPONENTS + 3] = bitangent_x;
                tangents_and_bitangents[facet * 6 + vertex * NUM_OF_COMPONENTS + 4] = bitangent_y;
                tangents_and_bitangents[facet * 6 + vertex * NUM_OF_COMPONENTS + 5] = bitangent_z;
            }
        }
    }
    let mut tans_and_bitans_buf = 0;
    gl::GenBuffers(1, &mut tans_and_bitans_buf);
    gl::BindBuffer(gl::ARRAY_BUFFER, tans_and_bitans_buf);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        tangents_and_bitangents.len() as isize * SIZE_OF_GL_FLOAT,
        tangents_and_bitangents.as_ptr() as *const c_void,
        gl::STATIC_DRAW,
    );
    gl::VertexAttribPointer(
        4,
        VEC3_COMPONENTS as _,
        gl::FLOAT,
        gl::FALSE,
        (NUM_OF_COMPONENTS * USIZE_OF_GL_FLOAT) as _,
        0 as *const c_void,
    );
    gl::VertexAttribPointer(
        5,
        VEC3_COMPONENTS as _,
        gl::FLOAT,
        gl::FALSE,
        (NUM_OF_COMPONENTS * USIZE_OF_GL_FLOAT) as _,
        (VEC3_COMPONENTS * USIZE_OF_GL_FLOAT) as *const c_void,
    );
    gl::EnableVertexAttribArray(4);
    gl::EnableVertexAttribArray(5);
    gfx.add_array_buffer(tans_and_bitans_buf, "Cube tangents and bitangents");
    CUBE_TANGENTS_AND_BITANGENTS_BUF = tans_and_bitans_buf;
}
