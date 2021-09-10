mod blinn_phong;
mod gamma_correction;
mod normal_mapping;
mod point_shadows;
mod shadow_mapping;

pub use blinn_phong::*;
pub use gamma_correction::*;
pub use normal_mapping::*;
pub use point_shadows::*;
pub use shadow_mapping::*;

#[derive(Copy, Clone)]
pub struct Attenuation {
    pub constant_term: f32,
    pub linear_term: f32,
    pub quadratic_term: f32,
}
