use crate::draw::PostProcessingOption;
use crate::state_and_cfg::State;
use glfw::Key;

pub fn change_post_processing_option(state: &mut State, key: Key) {
    let mut code = state.post_processing_option.int_code();
    match key {
        Key::Left => {
            code -= 1;
            if code < 0 {
                code = 13
            }
        }
        Key::Right => {
            code += 1;
            if code > 13 {
                code = 0;
            }
        }
        _ => {}
    };
    state.post_processing_option = PostProcessingOption::from_int_code(code);
    println!(
        "Post-processing option: {}",
        state.post_processing_option.to_string(),
    );
}
