use macroquad::{
    color::BLACK, input::{is_mouse_button_down, mouse_position, MouseButton}, math::Vec2, shapes::draw_circle
};

use crate::{camera::Camera, canvas_state::{CanvasState, DrawnPoint}};

#[derive(Copy, Clone, PartialEq)]
pub enum UserActionMode {
    Grab,
    Draw,
    Erase,
}

pub fn observe_user_action(camera: &mut Camera, mode: &UserActionMode, mut state: CanvasState) -> CanvasState {
    match mode {
        UserActionMode::Grab => {
            let (is_dragging, last_mouse_position) = handle_dragging(camera, state.is_dragging, state.last_mouse_position);
            state.is_dragging = is_dragging;
            state.last_mouse_position = last_mouse_position;
        }
        UserActionMode::Draw => {
            handle_drawing(&mut state, camera);
        }
        UserActionMode::Erase => {
            handle_erasing(&mut state, camera);
        }
    }
    state
}

pub fn handle_dragging(
    camera: &mut Camera,
    mut is_dragging: bool,
    mut last_mouse_position: Vec2,
) -> (bool, Vec2) {
    if is_mouse_button_down(MouseButton::Left) {
        if !is_dragging {
            is_dragging = true;
            last_mouse_position = mouse_position().into();
        }
        let current_mouse_position: Vec2 = mouse_position().into();
        let delta = (current_mouse_position - last_mouse_position) / camera.zoom;
        camera.position -= delta;
        last_mouse_position = current_mouse_position;
    } else {
        is_dragging = false;
    }
    (is_dragging, last_mouse_position)
}

fn handle_drawing(state: &mut CanvasState, camera: &Camera) {
    if is_mouse_button_down(MouseButton::Left) {
        let world_position = camera.screen_to_world(mouse_position().into());
        state.drawn_points.push(DrawnPoint {
            position: world_position,
            color: BLACK,
            size: 2.0, // Default size, can be adjusted later
        });
    }
}

fn handle_erasing(state: &mut CanvasState, camera: &Camera) {
    if is_mouse_button_down(MouseButton::Left) {
        let world_position = camera.screen_to_world(mouse_position().into());
        let erase_radius = 10.0; // Default size, can be adjusted later
        state.drawn_points.retain(|point| {
            Vec2::distance(point.position, world_position) > erase_radius
        });
    }
}

pub fn draw_canvas(state: &CanvasState, camera: &Camera) {
    for point in &state.drawn_points {
        let screen_position = camera.world_to_screen(point.position);
        draw_circle(screen_position.x, screen_position.y, point.size * camera.zoom, point.color);
    }
}