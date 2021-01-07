use matrix::{Matrix4x4, Vector3};

pub struct Camera {
    pub position: Vector3<f32>,
    pub direction: Vector3<f32>,
    pub world_up_direction: Vector3<f32>,
    pub look_at_matrix: Matrix4x4<f32>,
    pub yaw: f32,
    pub pitch: f32,
}

impl Camera {
    pub fn new() -> Camera {
        let position = Vector3::new(0.0, 0.0, 3.0);
        let direction = Vector3::new(0.0, 0.0, -1.0);
        let world_up_direction = Vector3::new(0.0, 1.0, 0.0);
        let look_at_matrix = Camera::calculate_look_at_matrix(
            position, /* Rustfmt force vertical formatting */
            direction,
            world_up_direction,
        );
        Camera {
            look_at_matrix,
            position,
            direction,
            world_up_direction,
            yaw: -90.0,
            pitch: 0.0,
        }
    }

    pub fn recalculate_look_at_matrix(&mut self) {
        self.look_at_matrix = Camera::calculate_look_at_matrix(
            self.position,
            self.direction + self.position,
            self.world_up_direction,
        );
    }

    fn calculate_look_at_matrix(
        camera_position: Vector3<f32>,
        camera_direction: Vector3<f32>,
        world_up_direction: Vector3<f32>,
    ) -> Matrix4x4<f32> {
        let camera_target_direction = !(camera_position - camera_direction);
        let camera_right = !(world_up_direction ^ camera_target_direction);
        let camera_up = camera_target_direction ^ camera_right;
        let (rx, ry, rz) = camera_right.get_components();
        let (ux, uy, uz) = camera_up.get_components();
        let (dx, dy, dz) = camera_target_direction.get_components();
        let rotation = Matrix4x4::from_array([
            [rx, ry, rz, 0.0],
            [ux, uy, uz, 0.0],
            [dx, dy, dz, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let translation = Matrix4x4::new_translation(
            -camera_position.x(), /* Rustfmt force vertical formatting */
            -camera_position.y(),
            -camera_position.z(),
        );
        rotation * translation
    }
}
