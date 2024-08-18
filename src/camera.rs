use macroquad::{
    math::{vec2, Mat4, Quat, Vec2, Vec3},
    window::{screen_height, screen_width},
};

pub struct Camera {
    pub position: Vec2,
    pub zoom: f32,
    pub last_mouse_position: Vec2,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            position: Vec2::ZERO,
            zoom: 1.0,
            last_mouse_position: Vec2::ZERO,
        }
    }

    pub fn adjust_zoom(&mut self, delta: f32) {
        let new_zoom = (self.zoom * (1.0 + delta)).clamp(0.1, 2.0);
        self.zoom = new_zoom;
    }

    pub fn world_to_screen(&self, world_pos: Vec2) -> Vec2 {
        (world_pos - self.position) * self.zoom + vec2(screen_width(), screen_height()) * 0.5
    }

    pub fn screen_to_world(&self, screen_pos: Vec2) -> Vec2 {
        (screen_pos - vec2(screen_width(), screen_height()) * 0.5) / self.zoom + self.position
    }

    pub fn get_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(
            Vec3::new(self.zoom, self.zoom, 1.0),
            Quat::IDENTITY,
            Vec3::new(-self.position.x, -self.position.y, 0.0)
        )
    }
}