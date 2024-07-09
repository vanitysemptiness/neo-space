use macroquad::{
    input::{
        is_mouse_button_down, is_mouse_button_pressed, is_mouse_button_released, mouse_position, mouse_wheel, MouseButton
    },
    math::Vec2,
    shapes::{draw_circle, draw_line},
};

use crate::{
    camera::Camera,
    canvas_state::{CanvasState, DrawnPoint},
    line_smoothing::catmull_rom_spline,
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
    state: &mut CanvasState,
) {
    handle_zoom(camera);

    match mode {
        UserActionMode::Grab => {
            let (is_dragging, last_mouse_position) =
                handle_dragging(camera, state.is_dragging, state.last_mouse_position);
            state.is_dragging = is_dragging;
            state.last_mouse_position = last_mouse_position;
        },
        UserActionMode::Draw => {
            handle_drawing(state, camera);
        },
        UserActionMode::Erase => {
            handle_erasing(state, camera);
        },
    }
}

fn handle_zoom(camera: &mut Camera) {
    let (_, wheel) = mouse_wheel();
    if wheel != 0.0 {
        let zoom_factor = if wheel > 0.0 { 0.1 } else { -0.1 };
        
        // Ignore zoom in attempts beyond 2x
        if !(camera.zoom >= 2.0 && zoom_factor > 0.0) {
            let new_zoom = (camera.zoom * (1.0 + zoom_factor)).clamp(0.1, 2.0);
            
            // Only apply zoom if it has changed
            if new_zoom != camera.zoom {
                let mouse_pos: Vec2 = mouse_position().into();
                let before = camera.screen_to_world(mouse_pos);
                
                camera.zoom = new_zoom;
                
                let after = camera.screen_to_world(mouse_pos);
                camera.position += before - after;
            }
        }
    }
}

pub fn handle_dragging(
    camera: &mut Camera,
    mut is_dragging: bool,
    mut last_mouse_position: Vec2,
) -> (bool, Vec2) {
    let current_mouse_position: Vec2 = mouse_position().into();

    if is_mouse_button_down(MouseButton::Left) {
        if !is_dragging {
            // Start of a new drag
            is_dragging = true;
            last_mouse_position = current_mouse_position;
        } else {
            // Continuing to drag
            let delta = (current_mouse_position - last_mouse_position) / camera.zoom;
            camera.position -= delta;
            last_mouse_position = current_mouse_position;
        }
    } else {
        // Mouse button is not down, end dragging if we were dragging
        is_dragging = false;
    }

    (is_dragging, last_mouse_position)
}

fn handle_erasing(state: &mut CanvasState, camera: &Camera) {
    if is_mouse_button_down(MouseButton::Left) {
        let world_position = camera.screen_to_world(mouse_position().into());
        let erase_radius = state.current_size * 2.0; // Adjust erase radius based on current size
        state
            .drawn_points
            .retain(|point| Vec2::distance(point.position, world_position) > erase_radius);
    }
}

fn handle_drawing(state: &mut CanvasState, camera: &Camera) {
    let current_position = camera.screen_to_world(mouse_position().into());

    if is_mouse_button_pressed(MouseButton::Left) {
        // Start a new line segment
        state.current_stroke.clear();
        state.current_stroke.push(current_position);
        // Add this line to mark the start of a new stroke
        state.drawn_points.push(DrawnPoint {
            position: current_position,
            color: state.current_color,
            size: state.current_size,
        });
    } else if is_mouse_button_down(MouseButton::Left) {
        // Add point to current stroke
        if let Some(&last_position) = state.current_stroke.last() {
            if (current_position - last_position).length() > 1.0 / camera.zoom {
                state.current_stroke.push(current_position);
            }
        }
    } else if is_mouse_button_released(MouseButton::Left) {
        // Apply smoothing and add to drawn points
        if state.current_stroke.len() >= 2 {
            let smoothed_points = catmull_rom_spline(&state.current_stroke);
            for point in smoothed_points {
                state.drawn_points.push(DrawnPoint {
                    position: point,
                    color: state.current_color,
                    size: state.current_size,
                });
            }
        }
        state.current_stroke.clear();
        // Add this line to mark the end of the current stroke
        state.drawn_points.push(DrawnPoint {
            position: Vec2::new(f32::NAN, f32::NAN),
            color: state.current_color,
            size: state.current_size,
        });
    }
}

// TODO: Rethink how this is organized, drawing doesn't really make sense here
pub fn draw_canvas(state: &CanvasState, camera: &Camera) {
    // Draw permanent lines
    let mut last_point: Option<&DrawnPoint> = None;
    for point in &state.drawn_points {
        if point.position.x.is_nan() || point.position.y.is_nan() {
            // This point marks the end of a stroke, reset last_point
            last_point = None;
            continue;
        }

        if let Some(last) = last_point {
            let start = camera.world_to_screen(last.position);
            let end = camera.world_to_screen(point.position);
            draw_line(
                start.x,
                start.y,
                end.x,
                end.y,
                last.size * camera.zoom,
                last.color,
            );
        }
        last_point = Some(point);
    }

    // Draw current stroke
    for points in state.current_stroke.windows(2) {
        let start = camera.world_to_screen(points[0]);
        let end = camera.world_to_screen(points[1]);
        draw_line(
            start.x,
            start.y,
            end.x,
            end.y,
            state.current_size * camera.zoom,
            state.current_color,
        );
    }
}
