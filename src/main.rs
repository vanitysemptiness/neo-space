use macroquad::prelude::*;

mod camera;
mod grid;
mod toolbar;
mod user_action_mode;
mod cursor;
mod info_hud;
mod canvas;

use camera::Camera;
use grid::Grid;
use toolbar::Toolbar;
use user_action_mode::UserActionMode;
use cursor::{Cursors, handle_cursor};
use info_hud::InfoHud;
use canvas::DrawingCanvas;
use std::rc::Rc;
use std::cell::RefCell;

const MIN_ZOOM: f32 = 0.1;
const MAX_ZOOM: f32 = 5.0;

struct State {
    camera: Camera,
    grid: Grid,
    drawing_canvas: Rc<RefCell<DrawingCanvas>>,
    toolbar: Toolbar,
    cursors: Cursors,
}

impl State {
    async fn new() -> Self {
        let drawing_canvas = Rc::new(RefCell::new(DrawingCanvas::new()));
        State {
            camera: Camera::new(),
            grid: Grid::new(),
            drawing_canvas: drawing_canvas.clone(),
            toolbar: Toolbar::new(drawing_canvas),
            cursors: Cursors {
                hand: load_texture("src/assets/hand_cursor.png").await.unwrap(),
                grab: load_texture("src/assets/grab_cursor.png").await.unwrap(),
            },
        }
    }

    fn update(&mut self) {
        self.handle_zoom();
        self.handle_user_action();
        self.handle_toolbar_input();
    }

    fn handle_zoom(&mut self) {
        let (wheel_x, wheel_y) = mouse_wheel();
        if is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl) {
            if wheel_y != 0.0 {
                let zoom_factor = 1.0 + (wheel_y * 0.1);
                let new_zoom = (self.camera.zoom * zoom_factor).clamp(MIN_ZOOM, MAX_ZOOM);
                
                let mouse_pos = Vec2::from(mouse_position());
                let before = self.camera.screen_to_world(mouse_pos);
                
                self.camera.zoom = new_zoom;
                
                let after = self.camera.screen_to_world(mouse_pos);
                self.camera.position += before - after;
            }
        } else {
            self.camera.position.y += wheel_y * 2.0 / self.camera.zoom;
            self.camera.position.x += wheel_x * 2.0 / self.camera.zoom;
        }
    }

    fn handle_user_action(&mut self) {
        let world_pos = self.camera.screen_to_world(mouse_position().into());

        match self.toolbar.mode {
            UserActionMode::Grab => self.handle_dragging(),
            UserActionMode::Draw => self.handle_drawing(world_pos),
            UserActionMode::Erase => self.handle_erasing(world_pos),
        }
    }

    fn handle_dragging(&mut self) {
        if is_mouse_button_down(MouseButton::Left) {
            let current_mouse_position: Vec2 = mouse_position().into();
            let delta = (current_mouse_position - self.camera.last_mouse_position) / self.camera.zoom;
            self.camera.position -= delta;
            self.camera.last_mouse_position = current_mouse_position;
        } else {
            self.camera.last_mouse_position = mouse_position().into();
        }
    }

    fn handle_drawing(&mut self, world_pos: Vec2) {
        let mut canvas = self.drawing_canvas.borrow_mut();
        if is_mouse_button_pressed(MouseButton::Left) {
            canvas.start_line(world_pos);
        } else if is_mouse_button_down(MouseButton::Left) {
            canvas.add_point(world_pos);
        } else if is_mouse_button_released(MouseButton::Left) {
            canvas.end_line();
        }
    }

    fn handle_erasing(&mut self, world_pos: Vec2) {
        if is_mouse_button_down(MouseButton::Left) {
            let erase_radius = 10.0 / self.camera.zoom;
            self.drawing_canvas.as_ref().borrow_mut().erase_at(world_pos, erase_radius);
        }
    }

    fn handle_toolbar_input(&mut self) {
        if let Some(new_mode) = self.toolbar.handle_input() {
            self.toolbar.mode = new_mode;
        }
    }

    fn draw(&mut self) {
        clear_background(grid::BACKGROUND_COLOR);
        self.grid.draw(&self.camera);
        self.drawing_canvas.borrow_mut().draw(&self.camera);
        self.toolbar.draw();
        handle_cursor(&self.toolbar.mode, &self.drawing_canvas.borrow(), &self.cursors);
        InfoHud::draw(&self.camera);
    }
}

#[macroquad::main("neo-space 🪐")]
async fn main() {
    let mut state = State::new().await;

    loop {
        state.update();
        state.draw();
        next_frame().await
    }
}