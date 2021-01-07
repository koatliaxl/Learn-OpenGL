use crate::gl;
use crate::state_and_cfg::{GlData, State};
use matrix::{Matrix4x4, Vector3};
use std::ffi::c_void;

#[allow(dead_code)]
pub unsafe fn draw_cubes(
    gfx: &GlData,
    state: &mut State,
    view_matrix: &Matrix4x4<f32>,
    projection_matrix: &Matrix4x4<f32>,
    time: f32,
    //time_sine: f32,
) {
    let time = time % 360.0;
    let time_sine = time.sin();
    let x_rotation = Matrix4x4::new_x_rotation(45.0);
    let rotation = Matrix4x4::new_y_rotation(time * 30.0) * &x_rotation;
    let translation = Matrix4x4::new_translation(0.0, 0.0, 0.0);
    let model_mat = translation * &rotation;

    let red_value = time_sine / 2.0 + 0.5;
    let green_value = 1.0 - red_value;

    gl::UseProgram(gfx.shader_programs[1]);
    gl::UniformMatrix4fv(
        gfx.get_var_loc("Model_mat", 1),
        1,
        gl::TRUE,
        model_mat.as_ptr(),
    );
    gl::UniformMatrix4fv(
        gfx.get_var_loc("View_mat", 1), /* Rustfmt force vertical formatting */
        1,
        gl::TRUE,
        view_matrix.as_ptr(),
    );
    gl::UniformMatrix4fv(
        gfx.get_var_loc("Projection_mat", 1),
        1,
        gl::TRUE,
        projection_matrix.as_ptr(),
    );
    gl::Uniform4f(
        gfx.get_var_loc("in_color", 1),
        red_value,
        green_value,
        0.0,
        1.0,
    );
    gl::Uniform1f(gfx.get_var_loc("Zoom", 1), state.zoom);
    gl::Uniform1f(gfx.get_var_loc("Mix", 1), state.mix);
    gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINES);

    gl::BindVertexArray(gfx.vertex_array_objects[1]);
    gl::ActiveTexture(gl::TEXTURE0);
    gl::BindTexture(gl::TEXTURE_2D, gfx.textures[0]);
    gl::ActiveTexture(gl::TEXTURE1);
    gl::BindTexture(gl::TEXTURE_2D, gfx.textures[1]);
    gl::ActiveTexture(gl::TEXTURE2);
    gl::BindTexture(gl::TEXTURE_2D, gfx.textures[2]);

    gl::DrawElements(
        gl::TRIANGLES, /* Rustfmt force vertical formatting */
        36,
        gl::UNSIGNED_INT,
        0 as *const c_void,
    );
    let cube_positions = [
        (2.0, 5.0, -15.0),
        (-1.5, -2.2, -2.5),
        (-3.8, -2.0, -12.3),
        (2.4, -0.4, -3.5),
        (-1.7, 3.0, -7.5),
        (1.3, -2.0, -2.5),
        (1.5, 2.0, -2.5),
        (1.5, 0.2, -1.5),
        (-1.3, 1.0, -1.5),
    ];
    for (i, pos) in cube_positions.iter().enumerate() {
        let translation = Matrix4x4::new_translation(pos.0, pos.1, pos.2);
        let rotation = Matrix4x4::new_rotation(
            time * 10.0 * i as f32,
            Vector3::new(1.0, 0.3, 0.5).normalize(),
        );
        let model_mat = translation * rotation;
        gl::UniformMatrix4fv(
            gfx.get_var_loc("Model_mat", 1),
            1,
            gl::TRUE,
            model_mat.as_ptr(),
        );
        gl::DrawElements(
            gl::TRIANGLES, /* Rustfmt force vertical formatting */
            36,
            gl::UNSIGNED_INT,
            0 as *const c_void,
        );
    }
}
