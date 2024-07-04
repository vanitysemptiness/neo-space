use macroquad::{
    color::Color, input::{is_key_down, mouse_position, KeyCode}, math::Vec2, shapes::draw_rectangle, window::{screen_height, screen_width}
};

use crate::camera::Camera;

const MIN_ZOOM: f32 = 0.1;
const MAX_ZOOM: f32 = 8.0;
const ZOOM_SPEED: f32 = 0.05;
const SCROLL_SPEED: f32 = 2.0;

pub struct ScrollBarConfig {
    pub visible_area_factor: f32,
    pub scrollbar_width: f32,
    pub background_color: Color,
    pub foreground_color: Color,
}

impl ScrollBarConfig {
    pub fn new() -> Self {
        Self {
            visible_area_factor: 10.0,
            scrollbar_width: 10.0,
            background_color: Color::new(0.9, 0.9, 0.9, 1.0),
            foreground_color: Color::new(0.7, 0.7, 0.7, 1.0),
        }
    }
}

/// The scrollbar has to display relative to how much in an infinite plane the user has scrolled
pub fn draw_scrollbar(scroll_bar_config: &ScrollBarConfig, camera: &Camera) {
    let scrollbar_width: f32 = scroll_bar_config.scrollbar_width;
    let background_color: Color = scroll_bar_config.background_color;
    let scrollbar_foreground_color: Color = scroll_bar_config.foreground_color;
    let visible_area_height: f32 = screen_height() * scroll_bar_config.visible_area_factor;
    let scrollbar_height: f32 =
        (screen_height() / visible_area_height * screen_height()).min(screen_height());
    let scroll_range: f32 = visible_area_height - screen_height();
    let normalized_scroll_pos: f32 =
        (camera.position.y % visible_area_height + visible_area_height) % visible_area_height;
    let scrollbar_position: f32 =
        (normalized_scroll_pos / scroll_range) * (screen_height() - scrollbar_height);
    draw_rectangle(
        screen_width() - scrollbar_width,
        0.0,
        scrollbar_width,
        screen_height(),
        background_color,
    );
    draw_rectangle(
        screen_width() - scrollbar_width,
        scrollbar_position,
        scrollbar_width,
        scrollbar_height,
        scrollbar_foreground_color,
    );
}

pub fn handle_scroll(scroll: &(f32, f32), camera: &mut Camera) {
    let &(wheel_x, wheel_y) = scroll;
    if is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl) {
        if wheel_y != 0.0 {
            let zoom_factor = 1.0 + (wheel_y * ZOOM_SPEED);
            let new_zoom = (camera.zoom * zoom_factor).clamp(MIN_ZOOM, MAX_ZOOM);
            
            let mouse_pos: Vec2 = mouse_position().into();
            let before = camera.screen_to_world(mouse_pos);
            
            camera.zoom = new_zoom;
            
            let after = camera.screen_to_world(mouse_pos);
            camera.position += before - after;
        }
    } else {
        // Scroll vertically and horizontally without Ctrl
        camera.position.y += wheel_y * SCROLL_SPEED / camera.zoom;
        camera.position.x += wheel_x * SCROLL_SPEED / camera.zoom;
    }
}