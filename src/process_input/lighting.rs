/*use crate::state_and_cfg::State;
use glfw::Key;

pub fn change_lighting_strength(state: &mut State, key: Key) {
    match key {
        Key::Num1 => {
            state.ambient_light_strength -= 0.01;
            if state.ambient_light_strength < 0.0 {
                state.ambient_light_strength = 0.0;
            }
            println!("Ambient light strength: {}", state.ambient_light_strength);
        }
        Key::Num2 => {
            state.ambient_light_strength += 0.01;
            println!("Ambient light strength: {}", state.ambient_light_strength);
        }
        Key::Num3 => {
            state.diffuse_light_strength -= 0.01;
            if state.diffuse_light_strength < 0.0 {
                state.diffuse_light_strength = 0.0;
            }
            println!("Diffuse light strength: {}", state.diffuse_light_strength);
        }
        Key::Num4 => {
            state.diffuse_light_strength += 0.01;
            println!("Diffuse light strength: {}", state.diffuse_light_strength);
        }
        Key::Num5 => {
            state.specular_light_strength -= 0.01;
            if state.specular_light_strength < 0.0 {
                state.specular_light_strength = 0.0;
            }
            println!("Specular light strength: {}", state.specular_light_strength);
        }
        Key::Num6 => {
            state.specular_light_strength += 0.01;
            println!("Specular light strength: {}", state.specular_light_strength);
        }
        Key::Num7 => {
            state.shininess /= 2.0;
            /*if state.shininess < 0.0 {
                state.shininess = 0.0;
            }*/
            println!("Shininess: {}", state.shininess);
        }
        Key::Num8 => {
            state.shininess *= 2.0;
            println!("Shininess: {}", state.shininess);
        }
        _ => {}
    }
}*/
