use macroquad::prelude::*;

mod camera;
mod grid;
mod canvas_state;
mod toolbar;
mod user_action_mode;
mod cursor;
mod info_hud;
mod line_smoothing;

use camera::Camera;
use grid::Grid;
use canvas_state::CanvasState;
use toolbar::Toolbar;
use user_action_mode::{UserActionMode, observe_user_action, draw_canvas};
use cursor::{Cursors, handle_cursor};
use info_hud::InfoHud;

const MIN_ZOOM: f32 = 0.1;
const MAX_ZOOM: f32 = 5.0;

struct State {
    camera: Camera,
    grid: Grid,
    canvas_state: CanvasState,
    toolbar: Toolbar,
    cursors: Cursors,
}

impl State {
    async fn new() -> Self {
        State {
            camera: Camera::new(),
            grid: Grid::new(),
            canvas_state: CanvasState::new(),
            toolbar: Toolbar::new(),
            cursors: Cursors {
                hand: load_texture("src/assets/hand_cursor.png").await.unwrap(),
                grab: load_texture("src/assets/grab_cursor.png").await.unwrap(),
            },
        }
    }

    fn update(&mut self) {
        // Handle scrolling
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

        // Update canvas state based on user action
        observe_user_action(&mut self.camera, &self.toolbar.mode, &mut self.canvas_state);

        // Handle toolbar input
        if let Some(new_mode) = self.toolbar.handle_input() {
            self.toolbar.mode = new_mode;
        }
    }

    fn draw(&self) {
        clear_background(grid::BACKGROUND_COLOR);

        // Draw grid
        self.grid.draw(&self.camera);

        // Draw canvas content
        draw_canvas(&self.canvas_state, &self.camera);

        // Draw toolbar
        self.toolbar.draw();

        // Handle cursor
        handle_cursor(&self.toolbar.mode, &self.canvas_state, &self.cursors);

        // Draw HUD
        InfoHud::draw(&self.camera);
    }
}

#[macroquad::main("Infinite Canvas")]
async fn main() {
    let mut state = State::new().await;

    loop {
        state.update();
        state.draw();

        next_frame().await
    }
}