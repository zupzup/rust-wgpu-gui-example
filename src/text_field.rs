use crate::rectangle::{RectPos, Rectangle};
use crate::text::Text;
use glyphon::FontSystem;
cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        use web_time::SystemTime;
    } else {
        use std::time::SystemTime;
    }
}

#[derive(Debug)]
pub struct TextFieldConfig {
    pub rect_pos: RectPos,
    pub fill_color: [f32; 3],
    pub fill_color_active: [f32; 3],
    pub border_color: [f32; 3],
    pub border_color_active: [f32; 3],
    pub text_color: glyphon::Color,
}

#[derive(Debug)]
pub struct TextField {
    pub text: Text,
    pub rectangle: Rectangle,
    pub content: String,
    pub active: bool,
    pub last_cursor_blink: Option<SystemTime>,
}

pub const CURSOR_BLINK_TIMEOUT_MS: u128 = 500;
const PADDING: u32 = 10;
const CURSOR_WIDTH: u32 = 2;

impl TextField {
    pub fn new(cfg: TextFieldConfig, font_system: &mut glyphon::FontSystem) -> Self {
        let padded_rect = RectPos {
            top: cfg.rect_pos.top + PADDING,
            left: cfg.rect_pos.left + PADDING,
            right: cfg.rect_pos.right - PADDING,
            bottom: cfg.rect_pos.bottom - PADDING,
        };
        Self {
            rectangle: Rectangle::new(
                cfg.rect_pos,
                cfg.fill_color,
                cfg.fill_color_active,
                cfg.border_color,
                cfg.border_color_active,
            ),
            text: Text::new(font_system, padded_rect, "", cfg.text_color, cfg.text_color),
            content: String::new(),
            active: false,
            last_cursor_blink: None,
        }
    }

    pub fn get_cursor(&self) -> Rectangle {
        let text_width = self.text.get_text_width();
        let rect_pos = self.rectangle.position;
        let left = if text_width.width > text_width.buffer_width {
            rect_pos.right - PADDING
        } else {
            rect_pos.left + text_width.width as u32 + PADDING
        };
        Rectangle::new(
            RectPos {
                top: rect_pos.top + PADDING,
                left,
                right: left + CURSOR_WIDTH,
                bottom: rect_pos.bottom - PADDING,
            },
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
        )
    }

    pub fn add_text(&mut self, font_system: &mut FontSystem, text: &str) {
        if self.active {
            self.content.push_str(text);
            self.text.set_text(font_system, &self.content);
        }
    }

    pub fn remove_character(&mut self, font_system: &mut FontSystem) {
        if self.active {
            self.content.pop();
            self.text.set_text(font_system, &self.content);
        }
    }

    pub fn set_active(&mut self) {
        self.active = true;
        if self.last_cursor_blink.is_none() {
            self.last_cursor_blink = Some(SystemTime::now());
        }
    }

    pub fn set_inactive(&mut self) {
        self.active = false;
        self.last_cursor_blink = None;
    }
}
