use crate::rectangle::{RectPos, Rectangle};
use crate::text::Text;
use winit::dpi::PhysicalPosition;

pub struct ButtonConfig {
    pub rect_pos: RectPos,
    pub fill_color: [f32; 3],
    pub fill_color_active: [f32; 3],
    pub border_color: [f32; 3],
    pub border_color_active: [f32; 3],
    pub text: &'static str,
    pub text_color: glyphon::Color,
    pub text_color_active: glyphon::Color,
    pub on_click: Box<dyn Fn()>,
}

pub struct Button {
    pub text: Text,
    pub rectangle: Rectangle,
    on_click: Box<dyn Fn()>,
}

impl Button {
    pub fn new(cfg: ButtonConfig, font_system: &mut glyphon::FontSystem) -> Self {
        Self {
            rectangle: Rectangle::new(
                cfg.rect_pos,
                cfg.fill_color,
                cfg.fill_color_active,
                cfg.border_color,
                cfg.border_color_active,
            ),
            text: Text::new(
                font_system,
                cfg.rect_pos,
                cfg.text,
                cfg.text_color,
                cfg.text_color_active,
            ),
            on_click: cfg.on_click,
        }
    }

    pub fn click(&mut self) {
        (self.on_click)()
    }

    pub fn is_hovered(&self, mouse_coords: PhysicalPosition<f64>) -> bool {
        self.rectangle.is_hovered(mouse_coords)
    }
}
