use crate::gl::types::GLint;
use crate::{gl, SIZE_OF_GL_FLOAT, SIZE_OF_GL_UNSIGNED_INT};
use image::RgbaImage;
use std::collections::HashMap;
use std::ffi::c_void;
use tobj::{Material, Mesh};
use TextureType::*;

pub struct Model {
    meshes: Vec<Mesh>,
    meshes_gl_data: Vec<Option<MeshOpenGLData>>,
    materials: Vec<Material>,
    textures_loaded: HashMap<String, Texture>,
    //shader_program_id: usize,
}

struct MeshOpenGLData {
    vertex_array_id: u32,
    vertex_buffer_id: u32,
    element_buffer_id: u32,
}

struct Texture {
    gl_id: Option<u32>,
    data: Option<RgbaImage>,
    gl_texture_unit: u32,
}

enum TextureType {
    Diffuse,
    Specular,
}

impl Model {
    pub fn new() -> Model {
        Model {
            meshes: Vec::new(),
            materials: Vec::new(),
            textures_loaded: HashMap::new(),
            meshes_gl_data: Vec::new(),
        }
    }

    pub fn load_model(&mut self, path: &str, obj_file_name: &str) {
        let obj_file_path = path.to_string() + &obj_file_name.to_string();
        let (models, materials) = tobj::load_obj(
            obj_file_path,
            true, /* Rustfmt force vertical formatting */
        )
        .expect("Failed to load model");
        for model in models {
            self.meshes.push(model.mesh);
        }
        for _ in 0..self.meshes.len() {
            self.meshes_gl_data.push(None)
        }
        for material in materials {
            self.materials.push(material)
        }
        for i in 0..self.materials.len() {
            self.load_texture(path, i, Diffuse);
            self.load_texture(path, i, Specular);
        }
    }

    fn load_texture(&mut self, path: &str, index: usize, tex_type: TextureType) {
        let (tex_file_name, gl_texture_unit) = match tex_type {
            Diffuse => (&self.materials[index].diffuse_texture, 0),
            Specular => (&self.materials[index].specular_texture, 1),
        };
        let tex_path = path.to_string() + tex_file_name;
        if !self.textures_loaded.contains_key(&tex_path) {
            let texture = image::open(&tex_path)
                .expect("Failed to load texture")
                //.flipv()
                .to_rgba8();
            self.textures_loaded.insert(
                tex_file_name.to_string(),
                Texture {
                    data: Some(texture),
                    gl_id: None,
                    gl_texture_unit,
                },
            );
            println!("Texture \"{}\" loaded", tex_file_name);
        } else {
            println!("Texture \"{}\" already loaded", tex_file_name);
        }
    }

    pub unsafe fn load_textures_to_gl(&mut self) {
        for mut texture in self.textures_loaded.values_mut() {
            if let Texture {
                gl_id: None,
                data: Some(image),
                ..
            } = texture
            {
                let (width, height) = image.dimensions();
                let mut texture_id = 0;
                gl::GenTextures(1, &mut texture_id);
                gl::BindTexture(gl::TEXTURE_2D, texture_id);
                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    gl::RGBA as i32,
                    width as i32,
                    height as i32,
                    0,
                    gl::RGBA,
                    gl::UNSIGNED_BYTE,
                    image.as_ptr() as *const c_void,
                );
                let (wrap_s, wrap_t, min_filter, mag_filter) = (
                    gl::CLAMP_TO_BORDER,
                    gl::CLAMP_TO_BORDER,
                    gl::LINEAR_MIPMAP_LINEAR,
                    gl::LINEAR,
                );
                gl::GenerateMipmap(gl::TEXTURE_2D);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wrap_s as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrap_t as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, min_filter as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, mag_filter as i32);
                texture.gl_id = Some(texture_id);
            }
        }
    }

    const POS_ATTRIB_LEN: GLint = 3;
    const NORMAL_ATTRIB_LEN: GLint = 3;
    const TEX_COORD_ATTRIB_LEN: GLint = 2;
    const VERTEX_LEN: GLint =
        Model::POS_ATTRIB_LEN + Model::NORMAL_ATTRIB_LEN + Model::TEX_COORD_ATTRIB_LEN;
    const STRIDE: GLint = Model::VERTEX_LEN * SIZE_OF_GL_FLOAT as i32;

    pub unsafe fn setup_draw(&mut self) {
        for (n, mesh) in self.meshes.iter_mut().enumerate() {
            let mut mesh_gl_data = MeshOpenGLData {
                vertex_array_id: 0,
                vertex_buffer_id: 0,
                element_buffer_id: 0,
            };
            gl::GenVertexArrays(1, &mut mesh_gl_data.vertex_array_id);
            gl::BindVertexArray(mesh_gl_data.vertex_array_id);

            gl::GenBuffers(1, &mut mesh_gl_data.vertex_buffer_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, mesh_gl_data.vertex_buffer_id);
            let mut mesh_vertices = Vec::with_capacity(
                mesh.positions.len() + mesh.normals.len() + mesh.texcoords.len(),
            );
            let (mut i, mut j) = (0, 0);
            while i < mesh.positions.len() {
                mesh_vertices.push(mesh.positions[i]);
                mesh_vertices.push(mesh.positions[i + 1]);
                mesh_vertices.push(mesh.positions[i + 2]);
                mesh_vertices.push(mesh.normals[i]);
                mesh_vertices.push(mesh.normals[i + 1]);
                mesh_vertices.push(mesh.normals[i + 2]);
                mesh_vertices.push(mesh.texcoords[j]);
                mesh_vertices.push(mesh.texcoords[j + 1]);
                i += 3;
                j += 2;
            }
            gl::BufferData(
                gl::ARRAY_BUFFER,
                mesh_vertices.len() as isize * SIZE_OF_GL_FLOAT,
                mesh_vertices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::GenBuffers(1, &mut mesh_gl_data.element_buffer_id);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, mesh_gl_data.element_buffer_id);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                mesh.indices.len() as isize * SIZE_OF_GL_UNSIGNED_INT,
                mesh.indices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                0,
                Model::POS_ATTRIB_LEN,
                gl::FLOAT,
                gl::FALSE,
                Model::STRIDE,
                0 as *const c_void,
            );
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                1,
                Model::NORMAL_ATTRIB_LEN,
                gl::FLOAT,
                gl::FALSE,
                Model::STRIDE,
                (SIZE_OF_GL_FLOAT as i32 * Model::POS_ATTRIB_LEN) as *const c_void,
            );
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                2,
                Model::TEX_COORD_ATTRIB_LEN,
                gl::FLOAT,
                gl::FALSE,
                Model::STRIDE,
                (SIZE_OF_GL_FLOAT as i32 * (Model::POS_ATTRIB_LEN + Model::NORMAL_ATTRIB_LEN))
                    as *const c_void,
            );
            gl::EnableVertexAttribArray(2);

            self.meshes_gl_data[n] = Some(mesh_gl_data);
        }
    }

    pub unsafe fn draw(&self) {
        for (i, mesh) in self.meshes.iter().enumerate() {
            if let Some(mesh_gl_data) = &self.meshes_gl_data[i] {
                gl::BindVertexArray(mesh_gl_data.vertex_array_id);
                if let Some(material_id) = mesh.material_id {
                    let material = &self.materials[material_id];
                    self.setup_texture_draw(&material.diffuse_texture);
                    self.setup_texture_draw(&material.specular_texture);
                }
                gl::DrawElements(
                    gl::TRIANGLES, /* Rustfmt force vertical formatting */
                    mesh.indices.len() as i32,
                    gl::UNSIGNED_INT,
                    0 as *const c_void,
                );
            }
        }
    }

    unsafe fn setup_texture_draw(&self, texture_name: &str) {
        if let Some(Texture {
            gl_id: Some(id),
            gl_texture_unit,
            ..
        }) = self.textures_loaded.get(texture_name)
        {
            gl::ActiveTexture(gl::TEXTURE0 + gl_texture_unit);
            gl::BindTexture(gl::TEXTURE_2D, *id);
        } else {
            println!("Material texture \"{}\" not present", texture_name);
        }
    }

    pub unsafe fn free_gl_resources(&mut self) {
        for opt in &mut self.meshes_gl_data {
            if let Some(mesh_gl_data) = opt {
                gl::DeleteVertexArrays(1, &mesh_gl_data.vertex_array_id);
                gl::DeleteBuffers(1, &mesh_gl_data.vertex_buffer_id);
                gl::DeleteBuffers(1, &mesh_gl_data.element_buffer_id);
            }
        }
        for tex in self.textures_loaded.values_mut() {
            if let Some(id) = tex.gl_id {
                gl::DeleteTextures(1, &id);
                tex.gl_id = None;
            }
        }
    }
}
