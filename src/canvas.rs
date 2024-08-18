use macroquad::{
    camera::{pop_camera_state, push_camera_state},
    color::{Color, BLACK},
    math::{vec2, Mat3, Vec2, Vec3},
    models::{draw_mesh, Mesh, Vertex},
};

use crate::camera::Camera;

pub struct DrawingCanvas {
    pub mesh: Mesh,
    current_line: Vec<Vec2>,
    pub current_color: Color,
    pub line_thickness: f32,
    needs_update: bool,
    last_camera_pos: Vec2,
    last_camera_zoom: f32,
}

impl DrawingCanvas {
    pub fn new() -> Self {
        DrawingCanvas {
            mesh: Mesh {
                vertices: Vec::new(),
                indices: Vec::new(),
                texture: None,
            },
            current_line: Vec::new(),
            current_color: BLACK,
            line_thickness: 2.0,
            needs_update: false,
            last_camera_pos: Vec2::ZERO,
            last_camera_zoom: 1.0,
        }
    }

    pub fn start_line(&mut self, position: Vec2) {
        self.current_line.clear();
        self.current_line.push(position);
    }

    pub fn add_point(&mut self, position: Vec2) {
        if let Some(&last_point) = self.current_line.last() {
            if (position - last_point).length() > 1.0 {
                self.current_line.push(position);
                self.needs_update = true;
            }
        } else {
            self.current_line.push(position);
            self.needs_update = true;
        }
    }

    pub fn end_line(&mut self) {
        if self.current_line.len() > 1 {
            self.update_mesh();
        }
        self.current_line.clear();
    }

    pub fn draw(&mut self, camera: &Camera) {
        if self.needs_update
            || camera.position != self.last_camera_pos
            || camera.zoom != self.last_camera_zoom
        {
            self.update_mesh();
            self.last_camera_pos = camera.position;
            self.last_camera_zoom = camera.zoom;
        }

        let transform = Mat3::from_scale_angle_translation(
            vec2(camera.zoom, camera.zoom),
            0.0,
            vec2(-camera.position.x, -camera.position.y),
        );

        push_camera_state();

        draw_mesh(&self.mesh);

        pop_camera_state();
    }

    pub fn set_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn set_thickness(&mut self, thickness: f32) {
        self.line_thickness = thickness;
    }

    fn update_mesh(&mut self) {
        self.mesh.vertices.clear();
        self.mesh.indices.clear();

        let mut index = 0;
        for window in self.current_line.windows(2) {
            let start = window[0];
            let end = window[1];
            let normal = (end - start).normalize().perp();
            let offset = normal * self.line_thickness * 0.5;

            let v1 = Vertex {
                position: Vec3::new(start.x + offset.x, start.y + offset.y, 0.0),
                uv: Vec2::ZERO,
                color: self.current_color,
            };
            let v2 = Vertex {
                position: Vec3::new(start.x - offset.x, start.y - offset.y, 0.0),
                uv: Vec2::ZERO,
                color: self.current_color,
            };
            let v3 = Vertex {
                position: Vec3::new(end.x + offset.x, end.y + offset.y, 0.0),
                uv: Vec2::ZERO,
                color: self.current_color,
            };
            let v4 = Vertex {
                position: Vec3::new(end.x - offset.x, end.y - offset.y, 0.0),
                uv: Vec2::ZERO,
                color: self.current_color,
            };

            self.mesh.vertices.extend_from_slice(&[v1, v2, v3, v4]);
            self.mesh.indices.extend_from_slice(&[
                index,
                index + 1,
                index + 2,
                index + 1,
                index + 3,
                index + 2,
            ]);

            index += 4;
        }

        self.needs_update = false;
    }

    pub fn erase_at(&mut self, position: Vec2, radius: f32) {
        let mut vertices_to_remove = Vec::new();
    
        for i in (0..self.mesh.vertices.len()).step_by(4) {
            let v1 = Vec2::new(self.mesh.vertices[i].position.x, self.mesh.vertices[i].position.y);
            let v2 = Vec2::new(self.mesh.vertices[i+2].position.x, self.mesh.vertices[i+2].position.y);
    
            let line_vector = v2 - v1;
            let t = ((position - v1).dot(line_vector)) / line_vector.length_squared();
            let closest_point = if t < 0.0 {
                v1
            } else if t > 1.0 {
                v2
            } else {
                v1 + line_vector * t
            };
    
            if (closest_point - position).length() <= radius {
                vertices_to_remove.push(i);
            }
        }
    
        // Remove the vertices and corresponding indices
        for &i in vertices_to_remove.iter().rev() {
            self.mesh.vertices.drain(i..i+4);
            let index_offset = i / 2 * 3;
            self.mesh.indices.drain(index_offset..index_offset+6);
        }
    
        // Update the indices to reflect the new vertex positions
        for (i, index) in self.mesh.indices.iter_mut().enumerate() {
            *index = (i / 6 * 4 + i % 6) as u16;
        }
    
        self.needs_update = true;
    }
}
