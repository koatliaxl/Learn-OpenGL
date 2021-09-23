use crate::gl;
use crate::state_and_cfg::GlData;

pub unsafe fn adv_data_use(gfx: &GlData) {
    gl::BindBuffer(gl::ARRAY_BUFFER, gfx.array_buffers[1]);
    print_gl_buffer(gl::ARRAY_BUFFER, 8 * 12, 8);
    #[allow(unused_imports)]
    use gl::{COPY_READ_BUFFER, COPY_WRITE_BUFFER};
    gl::BindBuffer(COPY_READ_BUFFER, gfx.array_buffers[4]);
    print_gl_buffer(COPY_READ_BUFFER, 3 * 8, 3);
    gl::CopyBufferSubData(gl::ARRAY_BUFFER, COPY_READ_BUFFER, 0, 0, 2 * 4);
    print_gl_buffer(COPY_READ_BUFFER, 3 * 8, 3);

    let ptr = gl::MapBuffer(COPY_READ_BUFFER, gl::WRITE_ONLY);
    let mut ptr = ptr as *mut f32;
    *ptr = 9.0;
    ptr = ptr.offset(2);
    *ptr = 9.9;
    if gl::UnmapBuffer(COPY_READ_BUFFER) == gl::TRUE {
        println!("\nGL data was mapped successfully");
    } else {
        println!("\nGL data mapping was unsuccessful");
    }
    print_gl_buffer(COPY_READ_BUFFER, 3 * 8, 3);
}

//todo generic
pub unsafe fn print_gl_buffer(target: gl::types::GLenum, size: usize, format: usize) {
    #[allow(unused_imports)]
    use gl::{READ_ONLY, READ_WRITE, WRITE_ONLY};
    let ptr = gl::MapBuffer(target, READ_ONLY);
    let mut vec = vec![0.0; size];
    (ptr as *const f32).copy_to(vec.as_mut_ptr(), size);
    let ret = gl::UnmapBuffer(target);
    if ret == gl::TRUE {
        println!("\nGL data was mapped successfully");
    } else {
        println!("\nGL data mapping was unsuccessful");
    }
    for i in 0..vec.len() {
        print!("{}, ", vec[i]);
        if format != 0 && (i + 1) % format == 0 {
            println!();
        }
    }
}
