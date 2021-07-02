mod blinn_phong;
mod gamma_correction;
mod shadow_mapping;

pub use blinn_phong::*;
pub use gamma_correction::*;
pub use shadow_mapping::*;

#[derive(Copy, Clone)]
pub struct Attenuation {
    pub constant_term: f32,
    pub linear_term: f32,
    pub quadratic_term: f32,
}
