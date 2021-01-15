use crate::gl;
use crate::shaders::*;

pub fn init_shader_programs() -> Vec<u32> {
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
        let shader_program_id =
            gen_shader_program(vertex_shader_id, fragment_shader_id, "Shader program 1");
        gl::DeleteShader(vertex_shader_id);
        gl::DeleteShader(fragment_shader_id);

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
        let shader_program_2_id =
            gen_shader_program(vertex_shader_2_id, fragment_shader_2_id, "Shader program 2");
        gl::DeleteShader(vertex_shader_2_id);
        gl::DeleteShader(fragment_shader_2_id);

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
        let shader_program_3_id =
            gen_shader_program(vertex_shader_3_id, fragment_shader_3_id, "Shader program 3");
        gl::DeleteShader(fragment_shader_3_id);
        let light_source_shader_program_id = gen_shader_program(
            vertex_shader_3_id,
            single_color_frag_shd_id,
            "Light Source Shader program",
        );
        gl::DeleteShader(vertex_shader_3_id);

        let model_vertex_shd_id = gen_shader_from_file(
            "shader_src/model_vertex_shd.glsl",
            gl::VERTEX_SHADER,
            "Model Vertex Shader",
        );
        let model_fragment_shd_id = gen_shader_from_file(
            "shader_src/model_fragment_shd.glsl",
            gl::FRAGMENT_SHADER,
            "Model Fragment Shader",
        );
        let model_shader_program = gen_shader_program(
            model_vertex_shd_id,
            model_fragment_shd_id,
            "Model Shader program",
        );

        gl::DeleteShader(model_fragment_shd_id);

        let depth_testing_fragment_shd = gen_shader_from_file(
            "shader_src/depth_testing_fragment_shd.glsl",
            gl::FRAGMENT_SHADER,
            "Depth Testing fragment shader",
        );
        let depth_testing_shd_program = gen_shader_program(
            model_vertex_shd_id,
            depth_testing_fragment_shd,
            "Depth Testing shader program",
        );
        gl::DeleteShader(depth_testing_fragment_shd);

        let outlining_shd_program = gen_shader_program(
            model_vertex_shd_id,
            single_color_frag_shd_id,
            "Object Outlining shader program",
        );
        gl::DeleteShader(single_color_frag_shd_id);

        let blending_frag_shd = gen_shader_from_file(
            "shader_src/blending_frag_shd.glsl",
            gl::FRAGMENT_SHADER,
            "Blending fragment shader",
        );
        let blending_shd_program = gen_shader_program(
            model_vertex_shd_id,
            blending_frag_shd,
            "Blending shader program",
        );
        gl::DeleteShader(model_vertex_shd_id);
        gl::DeleteShader(blending_frag_shd);
        vec![
            shader_program_id,
            shader_program_2_id,
            shader_program_3_id,
            light_source_shader_program_id,
            model_shader_program,
            depth_testing_shd_program,
            outlining_shd_program,
            blending_shd_program,
        ]
    }
}
