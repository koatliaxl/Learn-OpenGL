use crate::gl;
use crate::shaders::*;
use std::collections::HashMap;

pub fn init_shader_programs() -> (Vec<u32>, HashMap<String, usize>) {
    println!();
    unsafe {
        let vertex_shader_id = gen_shader(
            VERTEX_SHADER_SOURCE, /* Rustfmt force vertical formatting */
            gl::VERTEX_SHADER,
            "Vertex Shader",
        );
        let fragment_shader_id = gen_shader(
            FRAGMENT_SHADER_SOURCE,
            gl::FRAGMENT_SHADER,
            "Fragment Shader",
        );
        let vertex_shader_2_id = gen_shader_from_file(
            "shader_src/vertex_shd_2.glsl", /* Rustfmt force vertical formatting */
            gl::VERTEX_SHADER,
            "Vertex Shader 2",
        );
        let fragment_shader_2_id = gen_shader_from_file(
            "shader_src/fragment_shd_2.glsl",
            gl::FRAGMENT_SHADER,
            "Fragment Shader 2",
        );
        let vertex_shader_3_id = gen_shader_from_file(
            "shader_src/vertex_shd_3.glsl", /* Rustfmt force vertical formatting */
            gl::VERTEX_SHADER,
            "Vertex Shader 3",
        );
        let fragment_shader_3_id = gen_shader_from_file(
            "shader_src/fragment_shd_3.glsl",
            gl::FRAGMENT_SHADER,
            "Fragment Shader 3",
        );
        let single_color_frag_shd_id = gen_shader_from_file(
            "shader_src/single_color_frag_shd.glsl",
            gl::FRAGMENT_SHADER,
            "Single Color fragment shader",
        );
        let model_vertex_shd_id = gen_shader_from_file(
            "shader_src/model_vertex_shd.glsl",
            gl::VERTEX_SHADER,
            "Model vertex shader",
        );
        let model_fragment_shd_id = gen_shader_from_file(
            "shader_src/model_fragment_shd.glsl",
            gl::FRAGMENT_SHADER,
            "Model fragment shader",
        );
        let depth_testing_fragment_shd = gen_shader_from_file(
            "shader_src/depth_testing_fragment_shd.glsl",
            gl::FRAGMENT_SHADER,
            "Depth Testing fragment shader",
        );
        let blending_frag_shd = gen_shader_from_file(
            "shader_src/blending_frag_shd.glsl",
            gl::FRAGMENT_SHADER,
            "Blending fragment shader",
        );
        let post_processing_frag_shd = gen_shader_from_file(
            "shader_src/post_processing_frag_shd.glsl",
            gl::FRAGMENT_SHADER,
            "Post-processing fragment shader",
        );
        let cubemap_vertex_shd = gen_shader_from_file(
            "shader_src/cubemap_vertex_shd.glsl",
            gl::VERTEX_SHADER,
            "Cubemap vertex shader",
        );
        let cubemap_frag_shd = gen_shader_from_file(
            "shader_src/cubemap_frag_shd.glsl",
            gl::FRAGMENT_SHADER,
            "Cubemap fragment shader",
        );
        let shader_programs = [
            (vertex_shader_id, fragment_shader_id, "Shader 1"),
            (vertex_shader_2_id, fragment_shader_2_id, "Shader 2"),
            (vertex_shader_3_id, fragment_shader_3_id, "Shader 3"),
            (
                vertex_shader_3_id,
                single_color_frag_shd_id,
                "Light Source shader",
            ),
            (model_vertex_shd_id, model_fragment_shd_id, "Model shader"),
            (
                model_vertex_shd_id,
                depth_testing_fragment_shd,
                "Depth Testing shader",
            ),
            (
                model_vertex_shd_id,
                single_color_frag_shd_id,
                "Object Outlining shader",
            ),
            (model_vertex_shd_id, blending_frag_shd, "Blending shader"),
            (
                model_vertex_shd_id,
                post_processing_frag_shd,
                "Post-processing shader",
            ),
            (cubemap_vertex_shd, cubemap_frag_shd, "Cubemap shader"),
            (
                model_vertex_shd_id,
                cubemap_frag_shd,
                "Environment Mapping shader",
            ),
        ];
        let mut shader_program_ids = Vec::new();
        let mut shader_programs_index_keys = HashMap::new();
        for (vertex_shd, frag_shd, shd_program_name) in &shader_programs {
            let shd_program_id = gen_shader_program(*vertex_shd, *frag_shd, shd_program_name);
            shader_program_ids.push(shd_program_id);
            shader_programs_index_keys
                .insert(shd_program_name.to_string(), shader_program_ids.len() - 1);
        }
        shader_programs_index_keys.insert(
            "Default shader".to_string(),
            *shader_programs_index_keys.get("Blending shader").unwrap(),
        );
        gl::DeleteShader(vertex_shader_id);
        gl::DeleteShader(fragment_shader_id);
        gl::DeleteShader(vertex_shader_2_id);
        gl::DeleteShader(fragment_shader_2_id);
        gl::DeleteShader(fragment_shader_3_id);
        gl::DeleteShader(vertex_shader_3_id);
        gl::DeleteShader(model_fragment_shd_id);
        gl::DeleteShader(depth_testing_fragment_shd);
        gl::DeleteShader(single_color_frag_shd_id);
        gl::DeleteShader(blending_frag_shd);
        gl::DeleteShader(model_vertex_shd_id);
        gl::DeleteShader(post_processing_frag_shd);
        gl::DeleteShader(cubemap_vertex_shd);
        gl::DeleteShader(cubemap_frag_shd);

        (shader_program_ids, shader_programs_index_keys)
    }
}
