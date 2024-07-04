use macroquad::{color::{Color, BLACK}, math::Vec2};

pub struct CanvasState {
    pub is_dragging: bool,
    pub last_mouse_position: Vec2,
    pub drawn_points: Vec<DrawnPoint>,
    pub current_color: Color,
    pub current_size: f32,
}

pub struct DrawnPoint {
    pub position: Vec2,
    pub color: Color,
    pub size: f32,
}

impl CanvasState {
    pub fn new() -> Self {
        CanvasState {
            is_dragging: false,
            last_mouse_position: Vec2::ZERO,
            drawn_points: Vec::new(),
            current_color: BLACK,
            current_size: 2.0,
        }
    }
}