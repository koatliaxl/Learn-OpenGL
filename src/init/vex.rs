use crate::gl;
use crate::gl::types::{GLfloat, GLint, GLuint};
use ::opengl_learn::{SIZE_OF_GL_FLOAT, SIZE_OF_GL_UNSIGNED_INT};
use std::ffi::c_void;

pub fn init_vertex_array_objects() -> (Vec<u32>, Vec<u32>) {
    unsafe {
        let (vertex_array_obj_id, arr_buf_1_id) = triangle();
        let (vertex_array_obj_2_id, arr_buf_2_id, elem_arr_buf_1) = cube();
        let (vertex_array_obj_3_id, arr_buf_3_id) = cube_2();
        let (vertex_array_obj_4_id, arr_buf_4_id, elem_arr_buf_2) = skybox();
        let (vertex_array_obj_5_id, arr_buf_5_id) = points();
        (
            vec![
                vertex_array_obj_id,
                vertex_array_obj_2_id,
                vertex_array_obj_3_id,
                vertex_array_obj_4_id,
                vertex_array_obj_5_id,
            ],
            vec![
                arr_buf_1_id,
                arr_buf_2_id,
                elem_arr_buf_1,
                arr_buf_3_id,
                arr_buf_4_id,
                elem_arr_buf_2,
                arr_buf_5_id,
            ],
        )
    }
}

const USIZE_OF_GL_FLOAT: usize = std::mem::size_of::<GLfloat>();

const UB_POS_ATTRIB_LEN: usize = 3;
const UB_NORMAL_ATTRIB_LEN: usize = 3;
const UB_TEX_COORD_ATTRIB_LEN: usize = 2;
const UB_COLOR_ATTRIB_LEN: usize = 3;
const UB_VERTEX_LEN: usize =
    UB_POS_ATTRIB_LEN + UB_NORMAL_ATTRIB_LEN + UB_TEX_COORD_ATTRIB_LEN + UB_COLOR_ATTRIB_LEN;
const UB_STRIDE: usize = UB_VERTEX_LEN * USIZE_OF_GL_FLOAT;

unsafe fn points() -> (u32, u32) {
    const POINTS_NUM: usize = 4;
    let points: [((f32, f32, f32), (f32, f32, f32)); POINTS_NUM] = [
        ((0.5, 0.5, -0.5), (1.0, 0.0, 0.0)),
        ((0.5, -0.5, -0.5), (0.0, 1.0, 0.0)),
        ((-0.5, -0.5, -0.5), (0.0, 0.0, 1.0)),
        ((-0.5, 0.5, -0.5), (1.0, 1.0, 0.0)),
    ];
    let mut vertices_raw = [0.0; POINTS_NUM * UB_VERTEX_LEN];
    for i in 0..POINTS_NUM {
        let ((x, y, z), (r, g, b)) = points[i];
        vertices_raw[UB_VERTEX_LEN * i + 0] = x;
        vertices_raw[UB_VERTEX_LEN * i + 1] = y;
        vertices_raw[UB_VERTEX_LEN * i + 2] = z;
        let offset =
            UB_VERTEX_LEN * i + UB_POS_ATTRIB_LEN + UB_NORMAL_ATTRIB_LEN + UB_TEX_COORD_ATTRIB_LEN;
        vertices_raw[offset + 0] = r;
        vertices_raw[offset + 1] = g;
        vertices_raw[offset + 2] = b;
    }

    let mut vertex_array_obj_id = 0;
    gl::GenVertexArrays(1, &mut vertex_array_obj_id);
    gl::BindVertexArray(vertex_array_obj_id);

    let mut vertex_buf_obj_id = 0;
    gl::GenBuffers(1, &mut vertex_buf_obj_id);
    gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buf_obj_id);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        vertices_raw.len() as isize * SIZE_OF_GL_FLOAT,
        vertices_raw.as_ptr() as *const c_void,
        gl::STATIC_DRAW,
    );

    gl::VertexAttribPointer(
        0,
        UB_POS_ATTRIB_LEN as _,
        gl::FLOAT,
        gl::FALSE,
        UB_STRIDE as _,
        0 as *const c_void,
    );
    gl::EnableVertexAttribArray(0);
    gl::VertexAttribPointer(
        3,
        UB_COLOR_ATTRIB_LEN as _,
        gl::FLOAT,
        gl::FALSE,
        UB_STRIDE as _,
        ((UB_POS_ATTRIB_LEN + UB_NORMAL_ATTRIB_LEN + UB_TEX_COORD_ATTRIB_LEN) * USIZE_OF_GL_FLOAT)
            as *const c_void,
    );
    gl::EnableVertexAttribArray(3);

    (vertex_array_obj_id, vertex_buf_obj_id)
}

unsafe fn skybox() -> (u32, u32, u32) {
    const POS_ATTRIB_LEN: GLint = 3;
    const VERTEX_LEN: GLint = POS_ATTRIB_LEN;
    const VERTICES_NUM: usize = 8;
    let vertices: [(GLfloat, GLfloat, GLfloat); VERTICES_NUM] = [
        (1.0, 1.0, -1.0),
        (1.0, -1.0, -1.0),
        (-1.0, -1.0, -1.0),
        (-1.0, 1.0, -1.0),
        (1.0, 1.0, 1.0),
        (1.0, -1.0, 1.0),
        (-1.0, -1.0, 1.0),
        (-1.0, 1.0, 1.0),
    ];
    const FACETS: usize = 6;
    const INDICES_PER_FACET: usize = 6;
    let indices: [[GLuint; INDICES_PER_FACET]; FACETS] = [
        [0, 1, 3, 1, 2, 3],
        [0, 1, 4, 1, 4, 5],
        [0, 3, 4, 3, 4, 7],
        [2, 3, 7, 2, 6, 7],
        [1, 2, 5, 2, 5, 6],
        [4, 5, 7, 5, 6, 7],
    ];
    let mut vertices_raw = [0.0; VERTEX_LEN as usize * VERTICES_NUM];
    for n in 0..vertices.len() {
        let (x, y, z) = vertices[n];
        let i = n * VERTEX_LEN as usize;
        vertices_raw[i + 0] = x;
        vertices_raw[i + 1] = y;
        vertices_raw[i + 2] = z;
    }
    let mut indices_raw = [0; FACETS * INDICES_PER_FACET];
    for facet in 0..FACETS {
        for vertex in 0..INDICES_PER_FACET {
            indices_raw[facet * INDICES_PER_FACET + vertex] = indices[facet][vertex];
        }
    }

    let mut vertex_array_obj_id = 0;
    gl::GenVertexArrays(1, &mut vertex_array_obj_id);
    gl::BindVertexArray(vertex_array_obj_id);

    let mut vertex_buf_obj_id = 0;
    gl::GenBuffers(1, &mut vertex_buf_obj_id);
    gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buf_obj_id);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        vertices_raw.len() as isize * SIZE_OF_GL_FLOAT,
        vertices_raw.as_ptr() as *const c_void,
        gl::STATIC_DRAW,
    );

    let mut element_buf_obj_id = 0;
    gl::GenBuffers(1, &mut element_buf_obj_id);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, element_buf_obj_id);
    gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        indices_raw.len() as isize * SIZE_OF_GL_UNSIGNED_INT,
        indices_raw.as_ptr() as *const c_void,
        gl::STATIC_DRAW,
    );

    const STRIDE: GLint = VERTEX_LEN * SIZE_OF_GL_FLOAT as i32;
    gl::VertexAttribPointer(
        0,
        POS_ATTRIB_LEN,
        gl::FLOAT,
        gl::FALSE,
        STRIDE,
        0 as *const c_void,
    );
    gl::EnableVertexAttribArray(0);

    (vertex_array_obj_id, vertex_buf_obj_id, element_buf_obj_id)
}

unsafe fn cube_2() -> (u32, u32) {
    const POS_ATTRIB_LEN: GLint = 3;
    const NORMAL_ATTRIB_LEN: GLint = 3;
    const TEX_COORD_ATTRIB_LEN: GLint = 2;
    const VERTEX_LEN: GLint = POS_ATTRIB_LEN + NORMAL_ATTRIB_LEN + TEX_COORD_ATTRIB_LEN;
    const RAW_VERTICES_NUM: usize = 36;
    let vertices: [((GLfloat, GLfloat, GLfloat), (GLfloat, GLfloat)); 12] = [
        ((0.5, 0.5, -0.5), (1.0, 1.0)),
        ((0.5, -0.5, -0.5), (1.0, 0.0)),
        ((-0.5, -0.5, -0.5), (0.0, 0.0)),
        ((-0.5, 0.5, -0.5), (0.0, 1.0)),
        ((0.5, 0.5, 0.5), (0.0, 1.0)),
        ((0.5, -0.5, 0.5), (0.0, 0.0)),
        ((-0.5, -0.5, 0.5), (1.0, 0.0)),
        ((-0.5, 0.5, 0.5), (1.0, 1.0)),
        // Alternative vertices with different texture coord
        ((-0.5, 0.5, 0.5), (0.0, 0.0)),
        ((0.5, -0.5, 0.5), (1.0, 1.0)),
        ((0.5, 0.5, 0.5), (1.0, 0.0)),
        ((-0.5, -0.5, 0.5), (0.0, 1.0)),
    ];
    const FACETS: usize = 6;
    const FACET_INDICES: usize = 6;
    // Ordering applicable for face culling
    let indices: [[GLuint; FACET_INDICES]; FACETS] = [
        [0, 1, 3, 1, 2, 3],
        [4, 1, 0, 1, 4, 5],
        [0, 3, 10, 8, 10, 3],
        [7, 3, 2, 2, 6, 7],
        [9, 2, 1, 2, 9, 11],
        [7, 5, 4, 7, 6, 5],
    ];
    let normals = [
        (0.0, 0.0, -1.0),
        (1.0, 0.0, 0.0),
        (0.0, 1.0, 0.0),
        (-1.0, 0.0, 0.0),
        (0.0, -1.0, 0.0),
        (0.0, 0.0, 1.0),
    ];
    let mut vertices_raw = [0.0; VERTEX_LEN as usize * RAW_VERTICES_NUM];
    for n in 0..FACETS {
        for i in 0..FACET_INDICES {
            let ((x, y, z), (s, t)) = vertices[indices[n][i] as usize];
            let j = (n * FACET_INDICES + i) * VERTEX_LEN as usize;
            vertices_raw[j + 0] = x;
            vertices_raw[j + 1] = y;
            vertices_raw[j + 2] = z;
            let (nx, ny, nz) = normals[n];
            vertices_raw[j + 3] = nx;
            vertices_raw[j + 4] = ny;
            vertices_raw[j + 5] = nz;
            vertices_raw[j + 6] = s;
            vertices_raw[j + 7] = t;
        }
    }

    let mut vertex_array_obj_id = 0;
    gl::GenVertexArrays(1, &mut vertex_array_obj_id);
    gl::BindVertexArray(vertex_array_obj_id);

    let mut vertex_buf_obj_id = 0;
    gl::GenBuffers(1, &mut vertex_buf_obj_id);
    gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buf_obj_id);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        vertices_raw.len() as isize * SIZE_OF_GL_FLOAT,
        vertices_raw.as_ptr() as *const c_void,
        gl::STATIC_DRAW,
    );

    const STRIDE: GLint = VERTEX_LEN * SIZE_OF_GL_FLOAT as i32;
    gl::VertexAttribPointer(
        0,
        POS_ATTRIB_LEN,
        gl::FLOAT,
        gl::FALSE,
        STRIDE,
        0 as *const c_void,
    );
    gl::EnableVertexAttribArray(0);
    gl::VertexAttribPointer(
        1,
        NORMAL_ATTRIB_LEN,
        gl::FLOAT,
        gl::FALSE,
        STRIDE,
        (SIZE_OF_GL_FLOAT as i32 * POS_ATTRIB_LEN) as *const c_void,
    );
    gl::EnableVertexAttribArray(1);
    gl::VertexAttribPointer(
        2,
        TEX_COORD_ATTRIB_LEN,
        gl::FLOAT,
        gl::FALSE,
        STRIDE,
        (SIZE_OF_GL_FLOAT as i32 * (POS_ATTRIB_LEN + NORMAL_ATTRIB_LEN)) as *const c_void,
    );
    gl::EnableVertexAttribArray(2);

    (vertex_array_obj_id, vertex_buf_obj_id)
}

unsafe fn cube() -> (u32, u32, u32) {
    const POS_ATTRIB_LEN: GLint = 3;
    const COLOR_ATTRIB_LEN: GLint = 3;
    const TEX_COORD_ATTRIB_LEN: GLint = 2;
    const VERTEX_LEN: GLint = POS_ATTRIB_LEN + COLOR_ATTRIB_LEN + TEX_COORD_ATTRIB_LEN;
    const VERTICES_NUM: usize = 12;
    let vertices: [(
        (GLfloat, GLfloat, GLfloat),
        (GLfloat, GLfloat, GLfloat),
        (GLfloat, GLfloat),
    ); VERTICES_NUM] = [
        ((0.5, 0.5, -0.5), (1.0, 0.0, 0.0), (2.0, 2.0)),
        ((0.5, -0.5, -0.5), (0.0, 1.0, 0.0), (2.0, 0.0)),
        ((-0.5, -0.5, -0.5), (0.0, 0.0, 1.0), (0.0, 0.0)),
        ((-0.5, 0.5, -0.5), (1.0, 1.0, 0.0), (0.0, 2.0)),
        ((0.5, 0.5, 0.5), (1.0, 0.0, 0.0), (0.0, 2.0)),
        ((0.5, -0.5, 0.5), (0.0, 1.0, 0.0), (0.0, 0.0)),
        ((-0.5, -0.5, 0.5), (0.0, 0.0, 1.0), (2.0, 0.0)),
        ((-0.5, 0.5, 0.5), (1.0, 1.0, 0.0), (2.0, 2.0)),
        // Alternative vertices with different texture coord
        ((-0.5, 0.5, 0.5), (1.0, 1.0, 0.0), (0.0, 0.0)),
        ((0.5, -0.5, 0.5), (0.0, 1.0, 0.0), (2.0, 2.0)),
        ((0.5, 0.5, 0.5), (1.0, 0.0, 0.0), (2.0, 0.0)),
        ((-0.5, -0.5, 0.5), (0.0, 0.0, 1.0), (0.0, 2.0)),
    ];
    const FACETS: usize = 6;
    const FACET_INDICES: usize = 6;
    let indices: [[GLuint; FACET_INDICES]; FACETS] = [
        [0, 1, 3, 1, 2, 3],
        [0, 1, 4, 1, 4, 5],
        //[0, 3, 4, 3, 4, 7],
        [2, 3, 7, 2, 6, 7],
        //[1, 2, 5, 2, 5, 6],
        [4, 5, 7, 5, 6, 7],
        [0, 3, 10, 3, 10, 8],
        [1, 2, 9, 2, 9, 11],
    ];
    let mut vertices_raw = [0.0; VERTEX_LEN as usize * VERTICES_NUM];
    for n in 0..vertices.len() {
        let ((x, y, z), (r, g, b), (s, t)) = vertices[n];
        let i = n * VERTEX_LEN as usize;
        vertices_raw[i + 0] = x;
        vertices_raw[i + 1] = y;
        vertices_raw[i + 2] = z;
        vertices_raw[i + 3] = r;
        vertices_raw[i + 4] = g;
        vertices_raw[i + 5] = b;
        vertices_raw[i + 6] = s;
        vertices_raw[i + 7] = t;
    }
    let mut indices_raw = [0; FACETS * FACET_INDICES];
    for facet in 0..FACETS {
        for vertex in 0..FACET_INDICES {
            indices_raw[facet * FACET_INDICES + vertex] = indices[facet][vertex];
        }
    }

    let mut vertex_array_obj_id = 0;
    gl::GenVertexArrays(1, &mut vertex_array_obj_id);
    gl::BindVertexArray(vertex_array_obj_id);

    let mut vertex_buf_obj_id = 0;
    gl::GenBuffers(1, &mut vertex_buf_obj_id);
    gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buf_obj_id);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        vertices_raw.len() as isize * SIZE_OF_GL_FLOAT,
        vertices_raw.as_ptr() as *const c_void,
        gl::STATIC_DRAW,
    );

    let mut element_buf_obj_id = 0;
    gl::GenBuffers(1, &mut element_buf_obj_id);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, element_buf_obj_id);
    gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        indices_raw.len() as isize * SIZE_OF_GL_UNSIGNED_INT,
        indices_raw.as_ptr() as *const c_void,
        gl::STATIC_DRAW,
    );

    const STRIDE: GLint = VERTEX_LEN * SIZE_OF_GL_FLOAT as i32;
    gl::VertexAttribPointer(
        0,
        POS_ATTRIB_LEN,
        gl::FLOAT,
        gl::FALSE,
        STRIDE,
        0 as *const c_void,
    );
    gl::EnableVertexAttribArray(0);
    gl::VertexAttribPointer(
        1,
        COLOR_ATTRIB_LEN,
        gl::FLOAT,
        gl::FALSE,
        STRIDE,
        (SIZE_OF_GL_FLOAT as i32 * POS_ATTRIB_LEN) as *const c_void,
    );
    gl::EnableVertexAttribArray(1);
    gl::VertexAttribPointer(
        2,
        TEX_COORD_ATTRIB_LEN,
        gl::FLOAT,
        gl::FALSE,
        STRIDE,
        (SIZE_OF_GL_FLOAT as i32 * (POS_ATTRIB_LEN + COLOR_ATTRIB_LEN)) as *const c_void,
    );
    gl::EnableVertexAttribArray(2);

    (vertex_array_obj_id, vertex_buf_obj_id, element_buf_obj_id)
}

unsafe fn triangle() -> (u32, u32) {
    let vertices: [GLfloat; 15] = [
        0.5, -0.5, 1.0, 0.0, 0.0, -0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 0.5, 0.0, 0.0, 1.0,
    ];
    let position_attrib_len = 2;
    let color_attrib_len = 3;

    let mut vertex_buf_obj_id = 0;
    gl::GenBuffers(1, &mut vertex_buf_obj_id);
    gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buf_obj_id);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        vertices.len() as isize * SIZE_OF_GL_FLOAT,
        vertices.as_ptr() as *const c_void,
        gl::STATIC_DRAW,
    );

    let mut vertex_array_obj_id = 0;
    gl::GenVertexArrays(1, &mut vertex_array_obj_id);
    gl::BindVertexArray(vertex_array_obj_id);

    gl::VertexAttribPointer(
        0,
        position_attrib_len,
        gl::FLOAT,
        gl::FALSE,
        SIZE_OF_GL_FLOAT as i32 * (position_attrib_len + color_attrib_len),
        0 as *const c_void,
    );
    gl::EnableVertexAttribArray(0);
    gl::VertexAttribPointer(
        1,
        color_attrib_len,
        gl::FLOAT,
        gl::FALSE,
        SIZE_OF_GL_FLOAT as i32 * (position_attrib_len + color_attrib_len),
        (SIZE_OF_GL_FLOAT * position_attrib_len as isize) as *const c_void,
    );
    gl::EnableVertexAttribArray(1);

    /*gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    gl::BindVertexArray(0);*/
    (vertex_array_obj_id, vertex_buf_obj_id)
}
