pub const DEPTH_EMPTY_FRAG_SHADER: &str = "
    #version 330 core
    void main() {
        gl_FragDepth = gl_FragCoord.z;
    }
";

pub const SIMPLE_DEPTH_VERTEX_SHADER: &str = "
    #version 330 core
    layout (location = 0) in vec3 in_Position;
    uniform mat4 model_mat;
    uniform mat4 light_space_mat;
    void main() {
        gl_Position = light_space_mat * model_mat * vec4(in_Position, 1.0);
    }
";

pub const SINGLE_COLOR_ALPHA_FRAG_SHADER_SRC: &str = "
    #version 330 core
    out vec4 Frag_Color;
    uniform vec4 rgba;
    void main() {
        Frag_Color = rgba;
    }
";

pub const SINGLE_COLOR_FRAG_SHADER_SRC: &str = "
    #version 330 core
    out vec4 FragColor;
    uniform vec3 color;
    void main() {
        FragColor = vec4(color, 1.0);
    }
";

pub const FRAGMENT_SHADER_4_SRC: &str = "
    #version 330 core
    out vec4 Frag_Color;
    in vec3 Color;
    void main() {
        Frag_Color = vec4(Color, 0.0);
    }";

pub const UBO_FRAG_SHADER_SRC_1: &str = "
    #version 330 core
    out vec4 out_Color;
    void main() {
        out_Color = vec4(1.0, 0.0, 0.0, 1.0);
    }";

pub const UBO_FRAG_SHADER_SRC_2: &str = "
    #version 330 core
    out vec4 out_Color;
    void main() {
        out_Color = vec4(0.0, 1.0, 0.0, 1.0);
    }";

pub const UBO_FRAG_SHADER_SRC_3: &str = "
    #version 330 core
    out vec4 out_Color;
    void main() {
        out_Color = vec4(0.0, 0.0, 1.0, 1.0);
    }";

pub const VERTEX_SHADER_1_SRC: &str = "
    #version 330 core
    layout (location=0) in vec2 pos;
    layout (location=1) in vec3 color;
    out vec3 vertex_color;
    uniform float offset;
    void main() {
        gl_Position = vec4(pos.x/3.0 + offset, -pos.y/3.0+0.8, 0.0, 1.0);
        vertex_color = color;
    }";

pub const FRAGMENT_SHADER_1_SRC: &str = "
    #version 330 core
    out vec4 FragColor;
    in vec3 vertex_color;
    void main() {
        FragColor = vec4(vertex_color, 0.0);
    }";
