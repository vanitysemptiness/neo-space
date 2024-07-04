use canvas_state::CanvasState;
use cursor::{draw_cursor, handle_cursor, Cursors};
use macroquad::prelude::*;

mod camera;
use camera::Camera;

mod grid;
use grid::draw_grid;
use scrollbar::{draw_scrollbar, handle_scroll, ScrollBarConfig};
use user_action_mode::{observe_user_action, UserActionMode, draw_canvas};
use toolbar::Toolbar;

mod scrollbar;
mod user_action_mode;
mod canvas_state;
mod cursor;
mod info_hud;
mod toolbar;
use info_hud::display_hud;

#[macroquad::main(window_conf)]
async fn main() {
    let mut camera = Camera::new();
    let scroll_bar_config: ScrollBarConfig = ScrollBarConfig::new();
    let mut canvas_state = CanvasState::new();
    let mut toolbar = Toolbar::new();

    // Load cursor images
    let cursors = Cursors {
        hand: load_texture("src/assets/hand_cursor.png").await.unwrap(),
        grab: load_texture("src/assets/grab_cursor.png").await.unwrap(),
    };

    // Hide the default system cursor
    show_mouse(false);

    loop {
        clear_background(grid::BACKGROUND_COLOR);
        draw_grid(&camera);
        handle_scroll(&mouse_wheel(), &mut camera);
        draw_scrollbar(&scroll_bar_config, &camera);

        // Update canvas_state with current color and size from toolbar
        canvas_state.current_color = toolbar.current_color;
        canvas_state.current_size = toolbar.current_size;

        canvas_state = observe_user_action(&mut camera, &toolbar.mode, canvas_state);

        // Handle toolbar input
        toolbar.handle_input(); // no need to capture the mode, handled in toolbar

        // Draw the canvas content
        draw_canvas(&canvas_state, &camera);

        // Draw the toolbar
        toolbar.draw();

        // Draw the appropriate cursor
        draw_cursor(&toolbar.mode, &canvas_state, &cursors);
        // Handle cursor visibility and drawing
        handle_cursor(&toolbar.mode, &canvas_state, &cursors);
        display_hud(&camera);

        next_frame().await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Infinite Canvas".to_owned(),
        window_width: 1200,
        window_height: 800,
        // can add the frame rate here
        ..Default::default()
    }
}
