use macroquad::prelude::*;
use crate::camera::Camera;

const DOT_SIZE: f32 = 2.0;
const DOT_SPACING: f32 = 20.0;
pub const BACKGROUND_COLOR: Color = Color::new(0.95, 0.96, 0.98, 1.0);
const DOT_COLOR: Color = Color::new(0.4, 0.7, 0.9, 1.0);

pub struct Grid {
    dot_texture: Texture2D,
}

impl Grid {
    pub fn new() -> Self {
        let texture_size = 32;
        let mut image = Image::gen_image_color(texture_size, texture_size, Color::new(0.0, 0.0, 0.0, 0.0));
        let center = texture_size as f32 / 2.0;
        let radius = texture_size as f32 / 2.0;
        for y in 0..texture_size {
            for x in 0..texture_size {
                let dx = x as f32 - center;
                let dy = y as f32 - center;
                let distance = (dx * dx + dy * dy).sqrt();
                if distance <= radius {
                    let alpha = 1.0 - (distance / radius).powi(2);
                    image.set_pixel(x as u32, y as u32, Color::new(1.0, 1.0, 1.0, alpha));
                }
            }
        }
        Grid {
            dot_texture: Texture2D::from_image(&image),
        }
    }

    pub fn draw(&self, camera: &Camera) {
        let (screen_w, screen_h) = (screen_width(), screen_height());
        let top_left = camera.screen_to_world(Vec2::new(0.0, 0.0));
        let bottom_right = camera.screen_to_world(Vec2::new(screen_w, screen_h));

        let start_x = (top_left.x / DOT_SPACING).floor() as i32;
        let start_y = (top_left.y / DOT_SPACING).floor() as i32;
        let end_x = (bottom_right.x / DOT_SPACING).ceil() as i32;
        let end_y = (bottom_right.y / DOT_SPACING).ceil() as i32;

        let scaled_size = (DOT_SIZE * camera.zoom).max(0.5);
        let lod_factor = (1.0 / camera.zoom).ceil() as i32;

        let dest_size = Vec2::new(scaled_size, scaled_size);

        for x in (start_x..=end_x).step_by(lod_factor as usize) {
            for y in (start_y..=end_y).step_by(lod_factor as usize) {
                let world_pos = Vec2::new(x as f32 * DOT_SPACING, y as f32 * DOT_SPACING);
                let screen_pos = camera.world_to_screen(world_pos);
                if screen_pos.x >= -scaled_size && screen_pos.x <= screen_w + scaled_size &&
                   screen_pos.y >= -scaled_size && screen_pos.y <= screen_h + scaled_size {
                    draw_texture_ex(
                        self.dot_texture,
                        screen_pos.x - scaled_size / 2.0,
                        screen_pos.y - scaled_size / 2.0,
                        DOT_COLOR,
                        DrawTextureParams {
                            dest_size: Some(dest_size),
                            ..Default::default()
                        },
                    );
                }
            }
        }
    }
}