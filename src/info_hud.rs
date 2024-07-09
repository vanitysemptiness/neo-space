use macroquad::prelude::*;
use crate::camera::Camera;

pub struct InfoHud;

impl InfoHud {
    pub fn draw(camera: &Camera) {
        let text_color = Color::new(0.0, 0.0, 0.0, 0.7);
        draw_text(
            &format!("FPS: {}", get_fps()),
            10.0,
            20.0,
            20.0,
            text_color
        );
        draw_text(
            &format!("Zoom: {:.2}x", camera.zoom),
            10.0,
            40.0,
            20.0,
            text_color
        );
        draw_text(
            &format!("Pos: ({:.2}, {:.2})", camera.position.x, camera.position.y),
            10.0,
            60.0,
            20.0,
            text_color
        );
    }
}