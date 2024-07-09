use macroquad::prelude::*;
use crate::user_action_mode::UserActionMode;

const TOOLBAR_WIDTH: f32 = 100.0;
const BUTTON_HEIGHT: f32 = 60.0;
const BUTTON_PADDING: f32 = 10.0;
const COLOR_PICKER_HEIGHT: f32 = 120.0;
const SIZE_SLIDER_HEIGHT: f32 = 30.0;
const COLOR_BUTTON_SIZE: f32 = 30.0;
const COLOR_BUTTON_PADDING: f32 = 5.0;

pub struct Toolbar {
    pub mode: UserActionMode,
    pub current_color: Color,
    pub current_size: f32,
    colors: Vec<Color>,
    show_color_picker: bool,
}

impl Toolbar {
    pub fn new() -> Self {
        Toolbar {
            mode: UserActionMode::Grab,
            current_color: BLACK,
            current_size: 2.0,
            colors: vec![
                RED, GREEN, BLUE, YELLOW, PURPLE, ORANGE,
                PINK, BROWN, WHITE, GRAY, BLACK,
            ],
            show_color_picker: false,
        }
    }

    pub fn draw(&self) {
        let toolbar_color = Color::new(0.2, 0.2, 0.2, 1.0);
        draw_rectangle(0.0, 0.0, TOOLBAR_WIDTH, screen_height(), toolbar_color);

        self.draw_button("Grab", 0, self.mode == UserActionMode::Grab);
        self.draw_button("Draw", 1, self.mode == UserActionMode::Draw);
        self.draw_button("Erase", 2, self.mode == UserActionMode::Erase);

        if self.show_color_picker {
            self.draw_color_picker();
        }
        self.draw_size_slider();
        self.draw_current_color();
        self.draw_fps_counter();
    }

    fn draw_button(&self, label: &str, index: usize, is_active: bool) {
        let button_color = if is_active {
            Color::new(0.4, 0.4, 0.4, 1.0)
        } else {
            Color::new(0.3, 0.3, 0.3, 1.0)
        };

        let y = index as f32 * (BUTTON_HEIGHT + BUTTON_PADDING);
        draw_rectangle(0.0, y, TOOLBAR_WIDTH, BUTTON_HEIGHT, button_color);
        draw_text(
            label,
            5.0,
            y + BUTTON_HEIGHT / 2.0 + 5.0,
            20.0,
            Color::from_hex(0xffffff), // white
        );
    }

    fn draw_color_picker(&self) {
        let y = 3.0 * (BUTTON_HEIGHT + BUTTON_PADDING);
        draw_rectangle(0.0, y, TOOLBAR_WIDTH, COLOR_PICKER_HEIGHT, Color::new(0.3, 0.3, 0.3, 1.0));

        for (i, &color) in self.colors.iter().enumerate() {
            let row = i / 3;
            let col = i % 3;
            let x = col as f32 * (COLOR_BUTTON_SIZE + COLOR_BUTTON_PADDING) + COLOR_BUTTON_PADDING;
            let y = y + row as f32 * (COLOR_BUTTON_SIZE + COLOR_BUTTON_PADDING) + COLOR_BUTTON_PADDING;

            draw_rectangle(x, y, COLOR_BUTTON_SIZE, COLOR_BUTTON_SIZE, color);
            if color == self.current_color {
                draw_rectangle_lines(x, y, COLOR_BUTTON_SIZE, COLOR_BUTTON_SIZE, 2.0, WHITE);
            }
        }
    }

    fn draw_size_slider(&self) {
        let y = if self.show_color_picker {
            3.0 * (BUTTON_HEIGHT + BUTTON_PADDING) + COLOR_PICKER_HEIGHT + 10.0
        } else {
            3.0 * (BUTTON_HEIGHT + BUTTON_PADDING) + 10.0
        };
        draw_rectangle(10.0, y, TOOLBAR_WIDTH - 20.0, SIZE_SLIDER_HEIGHT, GRAY);
        draw_circle(10.0 + (TOOLBAR_WIDTH - 20.0) * (self.current_size / 20.0), y + SIZE_SLIDER_HEIGHT / 2.0, 5.0, WHITE);
    }

    fn draw_current_color(&self) {
        let y = if self.show_color_picker {
            3.0 * (BUTTON_HEIGHT + BUTTON_PADDING) + COLOR_PICKER_HEIGHT + SIZE_SLIDER_HEIGHT + 20.0
        } else {
            3.0 * (BUTTON_HEIGHT + BUTTON_PADDING) + SIZE_SLIDER_HEIGHT + 20.0
        };
        draw_rectangle(10.0, y, TOOLBAR_WIDTH - 20.0, 30.0, self.current_color);
        draw_rectangle_lines(10.0, y, TOOLBAR_WIDTH - 20.0, 30.0, 2.0, WHITE);
    }

    fn draw_fps_counter(&self) {
        let fps = get_fps() as i32;
        let fps_text = format!("FPS: {}", fps);
        let y = screen_height() - 30.0;
        draw_text(&fps_text, 10.0, y, 20.0, WHITE);
    }

    pub fn handle_input(&mut self) -> Option<UserActionMode> {
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            if mouse_x <= TOOLBAR_WIDTH {
                let index = (mouse_y / (BUTTON_HEIGHT + BUTTON_PADDING)) as usize;
                match index {
                    0 => {
                        self.mode = UserActionMode::Grab;
                        self.show_color_picker = false;
                    }
                    1 => {
                        self.mode = UserActionMode::Draw;
                        self.show_color_picker = true;
                    }
                    2 => {
                        self.mode = UserActionMode::Erase;
                        self.show_color_picker = false;
                    }
                    _ => {
                        if self.show_color_picker {
                            self.handle_color_picker_input(mouse_y);
                        }
                        self.handle_size_slider_input(mouse_y);
                        return None;
                    }
                };
                return Some(self.mode);
            }
        }
        None
    }

    fn handle_color_picker_input(&mut self, mouse_y: f32) {
        let color_picker_y = 3.0 * (BUTTON_HEIGHT + BUTTON_PADDING);
        if mouse_y >= color_picker_y && mouse_y < color_picker_y + COLOR_PICKER_HEIGHT {
            let (mouse_x, _) = mouse_position();
            let row = ((mouse_y - color_picker_y) / (COLOR_BUTTON_SIZE + COLOR_BUTTON_PADDING)) as usize;
            let col = (mouse_x / (COLOR_BUTTON_SIZE + COLOR_BUTTON_PADDING)) as usize;
            let index = row * 3 + col;
            
            if index < self.colors.len() {
                self.current_color = self.colors[index];
            }
        }
    }

    fn handle_size_slider_input(&mut self, mouse_y: f32) {
        let slider_y = if self.show_color_picker {
            3.0 * (BUTTON_HEIGHT + BUTTON_PADDING) + COLOR_PICKER_HEIGHT + 10.0
        } else {
            3.0 * (BUTTON_HEIGHT + BUTTON_PADDING) + 10.0
        };
        if mouse_y >= slider_y && mouse_y <= slider_y + SIZE_SLIDER_HEIGHT {
            let (mouse_x, _) = mouse_position();
            self.current_size = ((mouse_x - 10.0) / (TOOLBAR_WIDTH - 20.0) * 20.0).clamp(1.0, 20.0);
        }
    }
}