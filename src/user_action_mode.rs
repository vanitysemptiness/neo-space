use macroquad::prelude::*;

use crate::{
    camera::Camera,
    canvas::DrawingCanvas,
};

#[derive(Copy, Clone, PartialEq)]
pub enum UserActionMode {
    Grab,
    Draw,
    Erase,
}

pub fn observe_user_action(
    camera: &mut Camera,
    mode: &UserActionMode,
    canvas: &mut DrawingCanvas,
) {
    handle_zoom(camera);

    match mode {
        UserActionMode::Grab => {
            handle_dragging(camera);
        },
        UserActionMode::Draw => {
            handle_drawing(canvas, camera);
        },
        UserActionMode::Erase => {
            handle_erasing(canvas, camera);
        },
    }
}

fn handle_zoom(camera: &mut Camera) {
    let (_, wheel) = mouse_wheel();
    if wheel != 0.0 {
        let mouse_pos: Vec2 = mouse_position().into();
        let before = camera.screen_to_world(mouse_pos);
        
        camera.adjust_zoom(wheel * 0.1);
        
        let after = camera.screen_to_world(mouse_pos);
        camera.position += before - after;
    }
}

fn handle_dragging(camera: &mut Camera) {
    let current_mouse_position: Vec2 = mouse_position().into();
    
    if is_mouse_button_down(MouseButton::Left) {
        let delta = (current_mouse_position - camera.last_mouse_position) / camera.zoom;
        camera.position -= delta;
    }
    
    camera.last_mouse_position = current_mouse_position;
}

fn handle_erasing(canvas: &mut DrawingCanvas, camera: &Camera) {
    if is_mouse_button_down(MouseButton::Left) {
        let world_position = camera.screen_to_world(mouse_position().into());
        let erase_radius = 10.0 / camera.zoom; // Adjust this value as needed
        canvas.erase_at(world_position, erase_radius);
    }
}

fn handle_drawing(canvas: &mut DrawingCanvas, camera: &Camera) {
    let current_position = camera.screen_to_world(mouse_position().into());

    if is_mouse_button_pressed(MouseButton::Left) {
        canvas.start_line(current_position);
    } else if is_mouse_button_down(MouseButton::Left) {
        canvas.add_point(current_position);
    } else if is_mouse_button_released(MouseButton::Left) {
        canvas.end_line();
    }
}

// The draw_canvas function is no longer needed here as DrawingCanvas handles its own rendering