pub mod gl {
    include!("../gl/bindings.rs");
}
mod model;

pub use model::Model;

use crate::gl::types::{GLfloat, GLuint};

pub const SIZE_OF_GL_FLOAT: isize = std::mem::size_of::<GLfloat>() as isize;
pub const SIZE_OF_GL_UNSIGNED_INT: isize = std::mem::size_of::<GLuint>() as isize;

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
