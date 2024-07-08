use macroquad::{color::Color, math::{vec2, Vec2}, shapes::draw_circle, window::{screen_height, screen_width}};

use crate::camera::Camera;

const BASE_GRID_SIZE: f32 = 20.0;
const NORMAL_DOT_COLOR: Color = Color::new(0.7, 0.9, 1.0, 1.0);
const EMPHASIZED_DOT_COLOR: Color = Color::new(0.4, 0.7, 0.9, 1.0);
pub const BACKGROUND_COLOR: Color = Color::new(0.95, 0.96, 0.98, 1.0);
const DOT_SIZE: f32 = 1.0;
const EMPHASIS_INTERVAL: i32 = 4;

pub fn draw_grid(camera: &Camera) {
    let top_left = camera.screen_to_world(Vec2::ZERO);
    let bottom_right = camera.screen_to_world(vec2(screen_width(), screen_height()));

    let zoom_factor = camera.zoom;
    let grid_size = BASE_GRID_SIZE;

    let start_x = (top_left.x / grid_size).floor() * grid_size;
    let start_y = (top_left.y / grid_size).floor() * grid_size;
    let end_x = (bottom_right.x / grid_size).ceil() * grid_size;
    let end_y = (bottom_right.y / grid_size).ceil() * grid_size;

    let step = grid_size;

    let mut x = start_x;
    while x <= end_x {
        let mut y = start_y;
        while y <= end_y {
            let world_pos = vec2(x, y);
            let screen_pos = camera.world_to_screen(world_pos);
            
            let grid_x = (x / grid_size).round() as i32;
            let grid_y = (y / grid_size).round() as i32;
            
            let should_draw = if zoom_factor >= 1.0 {
                // When zooming in
                (grid_x % zoom_factor.floor() as i32 == 0 && grid_y % zoom_factor.floor() as i32 == 0) ||
                (grid_x % EMPHASIS_INTERVAL == 0 && grid_y % EMPHASIS_INTERVAL == 0)
            } else {
                // When zooming out
                let hide_factor = (1.0 / zoom_factor).ceil() as i32;
                grid_x % hide_factor == 0 && grid_y % hide_factor == 0
            };
            
            if should_draw {
                let is_emphasized = grid_x % EMPHASIS_INTERVAL == 0 && grid_y % EMPHASIS_INTERVAL == 0;
                let color = if is_emphasized { EMPHASIZED_DOT_COLOR } else { NORMAL_DOT_COLOR };
                
                draw_circle(screen_pos.x, screen_pos.y, DOT_SIZE, color);
            }
            
            y += step;
        }
        x += step;
    }
}