use macroquad::prelude::*;
use crate::user_action_mode::UserActionMode;
use crate::canvas::DrawingCanvas;

pub fn handle_cursor(mode: &UserActionMode, canvas: &DrawingCanvas, cursors: &Cursors) {
    let (mouse_x, mouse_y) = mouse_position();
    let window_width = screen_width();
    let window_height = screen_height();

    if mouse_x >= 0.0 && mouse_x < window_width && mouse_y >= 0.0 && mouse_y < window_height {
        // Mouse is inside the window
        show_mouse(false);
        draw_cursor(mode, canvas, cursors);
    } else {
        // Mouse is outside the window
        show_mouse(true);
    }
}

pub fn draw_cursor(mode: &UserActionMode, canvas: &DrawingCanvas, cursors: &Cursors) {
    let (mouse_x, mouse_y) = mouse_position();
    match mode {
        UserActionMode::Grab => {
            let cursor_texture = &cursors.hand; // Always use hand cursor for now
            draw_texture(
                *cursor_texture,
                mouse_x - cursor_texture.width() / 2.0,
                mouse_y - cursor_texture.height() / 2.0,
                WHITE,
            );
        }
        UserActionMode::Draw => {
            draw_circle_lines(mouse_x, mouse_y, canvas.line_thickness / 2.0, 1.0, canvas.current_color);
        }
        UserActionMode::Erase => {
            draw_circle_lines(mouse_x, mouse_y, canvas.line_thickness / 2.0, 1.0, BLACK);
            draw_circle(mouse_x, mouse_y, canvas.line_thickness / 2.0, Color::new(1.0, 1.0, 1.0, 0.2));
        }
    }
}

pub struct Cursors {
    pub hand: Texture2D,
    pub grab: Texture2D,
}