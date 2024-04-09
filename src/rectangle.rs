use crate::Vertex;
use winit::dpi::PhysicalPosition;

#[derive(Copy, Clone, Debug)]
pub struct RectPos {
    pub top: u32,
    pub left: u32,
    pub bottom: u32,
    pub right: u32,
}

#[derive(Debug, Clone)]
pub struct Rectangle {
    pub position: RectPos,
    color: [f32; 3],
    color_active: [f32; 3],
    border_color: [f32; 3],
    border_color_active: [f32; 3],
}

pub const NUM_INDICES: u32 = 6;

impl Rectangle {
    pub fn new(
        position: RectPos,
        color: [f32; 3],
        color_active: [f32; 3],
        border_color: [f32; 3],
        border_color_active: [f32; 3],
    ) -> Self {
        Self {
            color,
            color_active,
            border_color,
            border_color_active,
            position,
        }
    }

    pub fn vertices(
        &mut self,
        is_active: bool,
        size: winit::dpi::PhysicalSize<u32>,
    ) -> [Vertex; 4] {
        let top = 1.0 - (self.position.top as f32 / (size.height as f32 / 2.0));
        let left = (self.position.left as f32 / (size.width as f32 / 2.0)) - 1.0;
        let bottom = 1.0 - (self.position.bottom as f32 / (size.height as f32 / 2.0));
        let right = (self.position.right as f32 / (size.width as f32 / 2.0)) - 1.0;

        let rect = [
            self.position.top as f32,
            self.position.left as f32,
            self.position.bottom as f32,
            self.position.right as f32,
        ];

        let mut color = self.color;
        let mut border_color = self.border_color;
        if is_active {
            color = self.color_active;
            border_color = self.border_color_active;
        }

        //  -1, 1    1,1
        //  A--------D
        //  |        |
        //  |        |
        //  |        |
        //  B--------C
        //  -1, -1   1, -1
        [
            Vertex {
                // A
                position: [left, top, 0.0],
                color,
                rect,
                border_color,
            },
            Vertex {
                // B
                position: [left, bottom, 0.0],
                color,
                rect,
                border_color,
            },
            Vertex {
                // C
                position: [right, bottom, 0.0],
                color,
                rect,
                border_color,
            },
            Vertex {
                // D
                position: [right, top, 0.0],
                color,
                rect,
                border_color,
            },
        ]
    }

    pub fn indices(&self, base: u16) -> [u16; 6] {
        [base, 1 + base, 2 + base, base, 2 + base, 3 + base]
    }

    pub fn is_hovered(&self, mouse_coords: PhysicalPosition<f64>) -> bool {
        let rect_pos = self.position;
        mouse_coords.x > rect_pos.left as f64
            && mouse_coords.x < rect_pos.right as f64
            && mouse_coords.y > rect_pos.top as f64
            && mouse_coords.y < rect_pos.bottom as f64
    }
}
