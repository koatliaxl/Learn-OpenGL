use crate::gl;
use crate::shaders::*;
use crate::state_and_cfg::GlData;
use gl::{FRAGMENT_SHADER, GEOMETRY_SHADER, VERTEX_SHADER};

pub fn init_shader_programs(gl_data: &mut GlData) {
    println!();
    unsafe {
        let vertex_shader_id = gen_shader(
            VERTEX_SHADER_1_SRC, /* Rustfmt force vertical formatting */
            VERTEX_SHADER,
            "Vertex Shader",
        );
        let fragment_shader_id = gen_shader(
            FRAGMENT_SHADER_1_SRC,
            FRAGMENT_SHADER,
            "Fragment Shader", /* Rustfmt force vertical formatting */
        );
        let vertex_shader_2_id = gen_shader_from_file(
            "shader_src/vertex_shd_2.glsl", /* Rustfmt force vertical formatting */
            VERTEX_SHADER,
            "Vertex Shader 2",
        );
        let fragment_shader_2_id = gen_shader_from_file(
            "shader_src/fragment_shd_2.glsl",
            FRAGMENT_SHADER,
            "Fragment Shader 2",
        );
        let vertex_shader_3_id = gen_shader_from_file(
            "shader_src/vertex_shd_3.glsl", /* Rustfmt force vertical formatting */
            VERTEX_SHADER,
            "Vertex Shader 3",
        );
        let fragment_shader_3_id = gen_shader_from_file(
            "shader_src/fragment_shd_3.glsl",
            FRAGMENT_SHADER,
            "Fragment Shader 3",
        );
        let single_color_frag_shd_id = gen_shader(
            SINGLE_COLOR_FRAG_SHADER_SRC,
            FRAGMENT_SHADER,
            "Single Color fragment shader",
        );
        let model_vertex_shd_id = gen_shader_from_file(
            "shader_src/model_vertex_shd.glsl",
            VERTEX_SHADER,
            "Model vertex shader",
        );
        let model_fragment_shd_id = gen_shader_from_file(
            "shader_src/model_fragment_shd.glsl",
            FRAGMENT_SHADER,
            "Model fragment shader",
        );
        let depth_testing_fragment_shd = gen_shader_from_file(
            "shader_src/depth_testing_fragment_shd.glsl",
            FRAGMENT_SHADER,
            "Depth Testing fragment shader",
        );
        let blending_frag_shd = gen_shader_from_file(
            "shader_src/blending_frag_shd.glsl",
            FRAGMENT_SHADER,
            "Blending fragment shader",
        );
        let post_processing_frag_shd = gen_shader_from_file(
            "shader_src/post_processing_frag_shd.glsl",
            FRAGMENT_SHADER,
            "Post-processing fragment shader",
        );
        let cubemap_vertex_shd = gen_shader_from_file(
            "shader_src/cubemap_vertex_shd.glsl",
            VERTEX_SHADER,
            "Cubemap vertex shader",
        );
        let cubemap_frag_shd = gen_shader_from_file(
            "shader_src/cubemap_frag_shd.glsl",
            FRAGMENT_SHADER,
            "Cubemap fragment shader",
        );
        let ub_vertex_shd = gen_shader_from_file(
            "shader_src/ub_vertex_shd.glsl",
            VERTEX_SHADER,
            "UB vertex shader",
        );
        let ubo_frag_shd_1 = gen_shader(
            UBO_FRAG_SHADER_SRC_1,
            FRAGMENT_SHADER,
            "UBO Use fragment shader 1",
        );
        let ubo_frag_shd_2 = gen_shader(
            UBO_FRAG_SHADER_SRC_2,
            FRAGMENT_SHADER,
            "UBO Use fragment shader 2",
        );
        let ubo_frag_shd_3 = gen_shader(
            UBO_FRAG_SHADER_SRC_3,
            FRAGMENT_SHADER,
            "UBO Use fragment shader 3",
        );
        let geometry_shd_1 = gen_shader_from_file(
            "shader_src/geometry_shd_1.glsl",
            GEOMETRY_SHADER,
            "Geometry Shader 1",
        );
        let frag_shd_4 = gen_shader(
            FRAGMENT_SHADER_4_SRC, /* Rustfmt force vertical formatting */
            FRAGMENT_SHADER,
            "Fragment Shader 4",
        );
        let explode_effect_geometry_shd = gen_shader_from_file(
            "shader_src/explode_effect_geometry_shd.glsl",
            GEOMETRY_SHADER,
            "Explode Effect geometry shader",
        );
        let draw_normals_geometry_shd = gen_shader_from_file(
            "shader_src/draw_normals_geometry_shd.glsl",
            GEOMETRY_SHADER,
            "Draw Normals geometry shader",
        );
        let ib_vertex_shd = gen_shader_from_file(
            "shader_src/ib_vertex_shd.glsl",
            VERTEX_SHADER,
            "IB vertex shader",
        );
        let single_color_alpha_frag_shd = gen_shader(
            SINGLE_COLOR_ALPHA_FRAG_SHADER_SRC,
            FRAGMENT_SHADER,
            "Single Color+Alpha fragment shader",
        );
        let instancing_vertex_shd = gen_shader_from_file(
            "shader_src/instancing_vertex_shd.glsl",
            VERTEX_SHADER,
            "Instancing vertex shader",
        );
        let custom_anti_alias_frag = gen_shader_from_file(
            "shader_src/custom_anti_aliasing_frag.glsl",
            FRAGMENT_SHADER,
            "Custom Anti-aliasing fragment shader",
        );
        let adv_lighting_frag = gen_shader_from_file(
            "shader_src/adv_lighting_frag.glsl",
            FRAGMENT_SHADER,
            "Advanced Lighting fragment shader",
        );
        let simple_depth_shd = gen_shader(
            SIMPLE_DEPTH_VERTEX_SHADER,
            VERTEX_SHADER,
            "Simple Depth vertex shader",
        );
        let depth_empty_shd = gen_shader(
            DEPTH_EMPTY_FRAG_SHADER,
            FRAGMENT_SHADER,
            "Depth empty fragment shader",
        );
        let depth_visualization_frag = gen_shader_from_file(
            "shader_src/depth_visualization_frag.glsl",
            FRAGMENT_SHADER,
            "Depth Visualization fragment shader",
        );
        let shadow_mapping_vrtx = gen_shader_from_file(
            "shader_src/shadow_mapping_vrtx.glsl",
            VERTEX_SHADER,
            "Shadow Mapping vertex shader",
        );
        let shadow_mapping_frag = gen_shader_from_file(
            "shader_src/shadow_mapping_frag.glsl",
            FRAGMENT_SHADER,
            "Shadow Mapping fragment shader",
        );
        let depth_cubemap_geom = gen_shader_from_file(
            "shader_src/depth_cubemap_geom.glsl",
            GEOMETRY_SHADER,
            "Depth cubemap geometry shader",
        );
        let depth_cubemap_frag = gen_shader_from_file(
            "shader_src/depth_cubemap_frag.glsl",
            FRAGMENT_SHADER,
            "Depth cubemap fragment shader",
        );
        let point_shadows_frag = gen_shader_from_file(
            "shader_src/point_shadows_frag.glsl",
            FRAGMENT_SHADER,
            "Point Shadows fragment shader",
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
            // Safe to change ordering after this point
            (ub_vertex_shd, ubo_frag_shd_1, "UBO Use shader 1"),
            (ub_vertex_shd, ubo_frag_shd_2, "UBO Use shader 2"),
            (ub_vertex_shd, ubo_frag_shd_3, "UBO Use shader 3"),
            (ub_vertex_shd, blending_frag_shd, "UB Default shader"),
            (
                instancing_vertex_shd,
                blending_frag_shd,
                "Instancing shader",
            ),
            (
                model_vertex_shd_id,
                custom_anti_alias_frag,
                "Custom Anti-aliasing shader",
            ),
            (ub_vertex_shd, adv_lighting_frag, "Advanced Lighting shader"),
            (
                ub_vertex_shd,
                single_color_frag_shd_id,
                "Single Color shader",
            ),
            (simple_depth_shd, depth_empty_shd, "Depth/Shadow Map shader"),
            (
                model_vertex_shd_id,
                depth_visualization_frag,
                "Depth Visualization shader",
            ),
            (
                shadow_mapping_vrtx,
                shadow_mapping_frag,
                "Shadow Mapping shader",
            ),
            (ub_vertex_shd, point_shadows_frag, "Point Shadows shader"),
        ];
        println!();
        for (vertex_shd, frag_shd, shd_program_name) in &shader_programs {
            let shd_program_id = gen_shader_program(
                *vertex_shd,
                *frag_shd,
                shd_program_name, /* Rustfmt force vertical formatting */
            );
            gl_data.add_shader_program(shd_program_id, shd_program_name);
        }

        let geometry_shader_programs = [
            (
                ib_vertex_shd,
                frag_shd_4,
                geometry_shd_1,
                "Geometry Shader Use 1",
            ),
            (
                ib_vertex_shd,
                blending_frag_shd,
                explode_effect_geometry_shd,
                "Explode Effect shader",
            ),
            (
                ib_vertex_shd,
                single_color_alpha_frag_shd,
                draw_normals_geometry_shd,
                "Draw Normals shader",
            ),
            (
                ib_vertex_shd,
                depth_cubemap_frag,
                depth_cubemap_geom,
                "Depth cubemap shader",
            ),
        ];
        for (
            vertex_shd,
            frag_shd,
            geometry_shader,
            shd_program_name, /* Rustfmt force vertical formatting */
        ) in &geometry_shader_programs
        {
            let shd_program_id = gen_geometry_shader_program(
                *vertex_shd,
                *frag_shd,
                *geometry_shader,
                shd_program_name,
            );
            gl_data.add_shader_program(shd_program_id, shd_program_name);
        }

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
        gl::DeleteShader(ub_vertex_shd);
        gl::DeleteShader(ubo_frag_shd_1);
        gl::DeleteShader(ubo_frag_shd_2);
        gl::DeleteShader(ubo_frag_shd_3);
        gl::DeleteShader(geometry_shd_1);
        gl::DeleteShader(frag_shd_4);
        gl::DeleteShader(explode_effect_geometry_shd);
        gl::DeleteShader(draw_normals_geometry_shd);
        gl::DeleteShader(ib_vertex_shd);
        gl::DeleteShader(single_color_alpha_frag_shd);
        gl::DeleteShader(instancing_vertex_shd);
        gl::DeleteShader(custom_anti_alias_frag);
        gl::DeleteShader(adv_lighting_frag);
        gl::DeleteShader(simple_depth_shd);
        gl::DeleteShader(depth_empty_shd);
        gl::DeleteShader(depth_visualization_frag);
        gl::DeleteShader(shadow_mapping_vrtx);
        gl::DeleteShader(shadow_mapping_frag);
        gl::DeleteShader(depth_cubemap_geom);
        gl::DeleteShader(depth_cubemap_frag);
        gl::DeleteShader(point_shadows_frag);
    }
}
