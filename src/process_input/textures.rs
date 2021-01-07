use crate::state_and_cfg::State;
use glfw::Key;

pub fn change_textures(key: Key, state: &mut State) {
    let ref mut zoom = state.zoom;
    let ref mut mix = state.mix;
    match key {
        Key::Up => *zoom *= 1.04,
        Key::Down => *zoom /= 1.04,
        Key::Left => {
            *mix -= 0.03;
            if *mix < 0.0 {
                *mix = 0.0
            }
        }
        Key::Right => {
            *mix += 0.03;
            if *mix > 1.0 {
                *mix = 1.0
            }
        }
        _ => {}
    }
    match key {
        Key::Up | Key::Down => println!("Zoom: {}", zoom),
        Key::Left | Key::Right => println!("Mix ratio: {}", mix),
        _ => {}
    }
}
