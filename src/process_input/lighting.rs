use crate::state_and_cfg::State;
use glfw::Key;

pub fn change_lighting(state: &mut State, key: Key) {
    match key {
        /*Key::Num1 => {
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
        }*/
        Key::Num1 => {
            state.attenuation.constant_term -= 0.05;
            if state.attenuation.constant_term < 0.0 {
                state.attenuation.constant_term = 0.0;
            }
            println!(
                "Attenuation - Constant Term: {}",
                state.attenuation.constant_term
            );
        }
        Key::Num2 => {
            state.attenuation.constant_term += 0.05;
            println!(
                "Attenuation - Constant Term: {}",
                state.attenuation.constant_term
            );
        }

        Key::Num3 => {
            state.attenuation.linear_term -= 0.02;
            if state.attenuation.linear_term < 0.0 {
                state.attenuation.linear_term = 0.0;
            }
            println!(
                "Attenuation - Linear Term: {}",
                state.attenuation.linear_term
            );
        }
        Key::Num4 => {
            state.attenuation.linear_term += 0.02;
            println!(
                "Attenuation - Linear Term: {}",
                state.attenuation.linear_term
            );
        }

        Key::Num5 => {
            state.attenuation.quadratic_term -= 0.02;
            if state.attenuation.quadratic_term < 0.0 {
                state.attenuation.quadratic_term = 0.0;
            }
            println!(
                "Attenuation - Quadratic Term: {}",
                state.attenuation.quadratic_term
            );
        }
        Key::Num6 => {
            state.attenuation.quadratic_term += 0.02;
            println!(
                "Attenuation - Quadratic Term: {}",
                state.attenuation.quadratic_term
            );
        }

        Key::Minus => {
            state.shininess /= 2.0;
            /*if state.shininess < 0.0 {
                state.shininess = 0.0;
            }*/
            println!("Shininess: {}", state.shininess);
        }
        Key::Equal => {
            state.shininess *= 2.0;
            println!("Shininess: {}", state.shininess);
        }
        _ => {}
    }
}
