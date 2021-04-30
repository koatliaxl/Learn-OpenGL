pub mod gl {
    include!("../gl/bindings.rs");
}
mod model;

pub use model::Model;

use crate::gl::types::{GLfloat, GLuint};

pub const SIZE_OF_GL_FLOAT: isize = std::mem::size_of::<GLfloat>() as isize;
pub const SIZE_OF_GL_UNSIGNED_INT: isize = std::mem::size_of::<GLuint>() as isize;

pub unsafe fn check_framebuffer_gl_status() -> &'static str {
    match gl::CheckFramebufferStatus(gl::FRAMEBUFFER) {
        gl::FRAMEBUFFER_COMPLETE => "COMPLETE",
        gl::FRAMEBUFFER_UNDEFINED => "UNDEFINED",
        gl::FRAMEBUFFER_INCOMPLETE_ATTACHMENT => "INCOMPLETE_ATTACHMENT",
        gl::FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => "INCOMPLETE_MISSING_ATTACHMENT",
        gl::FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER => "INCOMPLETE_DRAW_BUFFER",
        gl::FRAMEBUFFER_INCOMPLETE_READ_BUFFER => "INCOMPLETE_READ_BUFFER",
        gl::FRAMEBUFFER_UNSUPPORTED => "UNSUPPORTED",
        gl::FRAMEBUFFER_INCOMPLETE_MULTISAMPLE => "INCOMPLETE_MULTISAMPLE",
        gl::FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS => "INCOMPLETE_LAYER_TARGETS",
        _ => "Error",
    }
}

/*struct Vertex {
    position: PositionAttribute,
    color: ColorAttribute,
    texture_coord: TextureCoord,
}
impl Vertex {
    fn new(pos: PositionAttribute, color: ColorAttribute, tex_coord: TextureCoord) -> Vertex {
        Vertex {
            position: pos,
            color,
            texture_coord: tex_coord,
        }
    }
}
struct PositionAttribute {
    x: GLfloat,
    y: GLfloat,
}
struct ColorAttribute {
    r: GLfloat,
    y: GLfloat,
    z: GLfloat,
}
struct TextureCoord {
    s: GLfloat,
    t: GLfloat,
}*/
