use crate::draw::IDENTITY_MATRIX;
use crate::gl;
use crate::gl::types::GLuint;
use crate::init::{
    CUBE2_INDICES, CUBE2_RAW_VERTICES_NUM, CUBE2_VERTICES, CUBE_FACETS, INDICES_PER_CUBE_FACET,
};
use crate::state_and_cfg::{GlData, State};
use mat_vec::{Matrix4x4, Vector3};
use opengl_learn::{SIZE_OF_GL_FLOAT, USIZE_OF_GL_FLOAT};
use std::ffi::c_void;

static mut CUBE_TANGENTS_AND_BITANGENTS_BUF: GLuint = 0;

pub struct NormalMappingSettings {
    enabled: bool,
    in_shader_bitangent_generation: bool,
}

pub unsafe fn draw_normal_mapping(gfx: &GlData, state: &State) {
    let shd_idx = match state.tangent_space_correction {
        true => gfx.get_shader_program_index("Normal Mapping shader"),
        false => gfx.get_shader_program_index("Advanced Lighting shader"),
    };
    gl::UseProgram(gfx.shader_programs[shd_idx]);
    gfx.set_uniform_vec3f("Viewer_Position", shd_idx, state.camera.position);
    gfx.set_uniform_1b("normal_mapping", shd_idx, state.normal_mapping);
    gfx.set_uniform_1f("Shininess", shd_idx, state.shininess);
    if state.tangent_space_correction {
        gfx.set_uniform_1b("generate_bitangents", shd_idx, state.normal_mapping);
    }
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

    let shd_idx = gfx.get_shader_program_index("Normal Mapping shader");
    gl::UseProgram(gfx.shader_programs[shd_idx]);
    gfx.set_uniform_mat4x4("model_mat", shd_idx, &IDENTITY_MATRIX);
    gfx.set_uniform_1i("Light_Sources_Num", shd_idx, 1);
    gfx.set_uniform_1i("normal_map", shd_idx, 1);
    gfx.set_uniform_vec3f("light_positions[0]", shd_idx, light_source_pos);
    gfx.set_uniform_3f("Light_Sources[0].color", shd_idx, 1.0, 1.0, 1.0);
    gfx.set_uniform_1f("attenuation_linear_term", shd_idx, 0.01);
    gfx.set_uniform_1f("attenuation_quadratic_term", shd_idx, 0.01);

    let shd_idx = gfx.get_shader_program_index("Advanced Lighting shader");
    gl::UseProgram(gfx.shader_programs[shd_idx]);
    gfx.set_uniform_1i("Light_Sources_Num", shd_idx, 1);
    gfx.set_uniform_mat4x4("model_mat", shd_idx, &IDENTITY_MATRIX);
    gfx.set_uniform_1i("normal_map", shd_idx, 1);
    gfx.set_uniform_vec3f("Light_Sources[0].position", shd_idx, light_source_pos);
    gfx.set_uniform_3f("Light_Sources[0].color", shd_idx, 1.0, 1.0, 1.0);
    gfx.set_uniform_1f("attenuation_linear_term", shd_idx, 0.01);
    gfx.set_uniform_1f("attenuation_quadratic_term", shd_idx, 0.01);

    let shd_idx = gfx.get_shader_program_index("Single Color shader");
    gl::UseProgram(gfx.shader_programs[shd_idx]);
    let scaling = Matrix4x4::new_uniform_scaling(0.1);
    let translation = Matrix4x4::new_translation_from_vec(light_source_pos);
    let model_mat = translation * scaling;
    gfx.set_uniform_mat4x4("model_mat", shd_idx, &model_mat);
    gfx.set_uniform_vec3f("color", shd_idx, Vector3::new(1.0, 1.0, 1.0));

    const VEC3_COMPONENTS: usize = 3;
    const COMPONENTS_PER_VERTEX: usize = VEC3_COMPONENTS * 2; // tangent + bitangent
    let mut tangents_and_bitangents = [0.0; CUBE2_RAW_VERTICES_NUM * COMPONENTS_PER_VERTEX];
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
            let edge_2 = pos3 - pos1;
            let delta_uv1 = tex_coord_1 - tex_coord_2;
            let delta_uv2 = tex_coord_3 - tex_coord_1;

            // reciprocal of determinant of delta texture coordinate matrix
            let rd = 1.0 / (delta_uv1.x() * delta_uv2.y() - delta_uv2.x() * delta_uv1.y());

            let tangent_x = rd * (delta_uv2.y() * edge_1.x() - delta_uv1.y() * edge_2.x());
            let tangent_y = rd * (delta_uv2.y() * edge_1.y() - delta_uv1.y() * edge_2.y());
            let tangent_z = rd * (delta_uv2.y() * edge_1.z() - delta_uv1.y() * edge_2.z());
            let bitangent_x = rd * (-delta_uv2.x() * edge_1.x() - delta_uv1.x() * edge_2.x());
            let bitangent_y = rd * (-delta_uv2.x() * edge_1.y() - delta_uv1.x() * edge_2.y());
            let bitangent_z = rd * (-delta_uv2.x() * edge_1.z() - delta_uv1.x() * edge_2.z());

            println!("\nfacet: {}, triangle: {}, rd: {}", facet, triangle, rd);
            println!("edge1: {:?}, edge2: {:?}", edge_1, edge_2);
            println!("delta_uv1: {:?}, delta_uv2: {:?}", delta_uv1, delta_uv2);
            println!("tangent: ({}, {}, {})", tangent_x, tangent_y, tangent_z);
            println!(
                "bitangent: ({}, {}, {})",
                bitangent_x, bitangent_y, bitangent_z
            );

            const VERTEXES_PER_FACET: usize = INDICES_PER_CUBE_FACET;
            const COMPONENTS_PER_FACET: usize = VERTEXES_PER_FACET * COMPONENTS_PER_VERTEX;
            for vertex in (triangle * 3)..INDICES_PER_CUBE_FACET / (2 - triangle) {
                tangents_and_bitangents
                    [facet * COMPONENTS_PER_FACET + vertex * COMPONENTS_PER_VERTEX + 0] = tangent_x;
                tangents_and_bitangents
                    [facet * COMPONENTS_PER_FACET + vertex * COMPONENTS_PER_VERTEX + 1] = tangent_y;
                tangents_and_bitangents
                    [facet * COMPONENTS_PER_FACET + vertex * COMPONENTS_PER_VERTEX + 2] = tangent_z;
                tangents_and_bitangents
                    [facet * COMPONENTS_PER_FACET + vertex * COMPONENTS_PER_VERTEX + 3] =
                    bitangent_x;
                tangents_and_bitangents
                    [facet * COMPONENTS_PER_FACET + vertex * COMPONENTS_PER_VERTEX + 4] =
                    bitangent_y;
                tangents_and_bitangents
                    [facet * COMPONENTS_PER_FACET + vertex * COMPONENTS_PER_VERTEX + 5] =
                    bitangent_z;
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
        (COMPONENTS_PER_VERTEX * USIZE_OF_GL_FLOAT) as _,
        0 as *const c_void,
    );
    gl::VertexAttribPointer(
        5,
        VEC3_COMPONENTS as _,
        gl::FLOAT,
        gl::FALSE,
        (COMPONENTS_PER_VERTEX * USIZE_OF_GL_FLOAT) as _,
        (VEC3_COMPONENTS * USIZE_OF_GL_FLOAT) as *const c_void,
    );
    gl::EnableVertexAttribArray(4);
    gl::EnableVertexAttribArray(5);
    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    gfx.add_array_buffer(tans_and_bitans_buf, "Cube tangents and bitangents");
    CUBE_TANGENTS_AND_BITANGENTS_BUF = tans_and_bitans_buf;

    gl::BindBuffer(gl::COPY_READ_BUFFER, CUBE_TANGENTS_AND_BITANGENTS_BUF);
    super::super::print_gl_buffer(
        gl::COPY_READ_BUFFER,
        CUBE2_RAW_VERTICES_NUM * COMPONENTS_PER_VERTEX,
        3,
    );
}
