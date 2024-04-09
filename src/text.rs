use crate::rectangle::RectPos;
use glyphon::{Attrs, Buffer, Color, Family, FontSystem, Metrics, Shaping, TextArea, TextBounds};

#[derive(Debug)]
pub struct TextWidth {
    pub width: f32,
    pub buffer_width: f32,
}

#[derive(Debug)]
pub struct Text {
    buffer: Buffer,
    rect_pos: RectPos,
    color: Color,
    color_active: Color,
}

const FONT_SIZE: f32 = 30.0;
const LINE_HEIGHT: f32 = 42.0;

impl Text {
    pub fn new(
        font_system: &mut FontSystem,
        rect_pos: RectPos,
        text: &str,
        color: Color,
        color_active: Color,
    ) -> Self {
        let mut buffer = Buffer::new(font_system, Metrics::new(FONT_SIZE, LINE_HEIGHT));

        buffer.set_size(
            font_system,
            (rect_pos.right - rect_pos.left) as f32,
            (rect_pos.bottom - rect_pos.top) as f32,
        );

        buffer.set_text(
            font_system,
            text,
            Attrs::new().family(Family::SansSerif),
            Shaping::Advanced,
        );

        buffer.lines.iter_mut().for_each(|line| {
            line.set_align(Some(glyphon::cosmic_text::Align::Center));
        });

        buffer.set_wrap(font_system, glyphon::Wrap::None);

        buffer.shape_until_scroll(font_system);

        Self {
            buffer,
            rect_pos,
            color,
            color_active,
        }
    }

    pub fn get_text_width(&self) -> TextWidth {
        TextWidth {
            width: self
                .buffer
                .layout_runs()
                .fold(0.0, |width, run| run.line_w.max(width)),
            buffer_width: self.buffer.size().0,
        }
    }

    pub fn set_text(&mut self, font_system: &mut FontSystem, text: &str) {
        self.buffer.set_text(
            font_system,
            text,
            Attrs::new().family(Family::SansSerif),
            Shaping::Advanced,
        );
    }

    fn top(&self) -> f32 {
        (self.rect_pos.bottom - (self.rect_pos.bottom - self.rect_pos.top) / 2) as f32
            - (self.buffer.metrics().line_height / 2.0)
    }

    fn bounds(&self) -> TextBounds {
        TextBounds {
            left: self.rect_pos.left as i32,
            top: self.rect_pos.top as i32,
            right: self.rect_pos.right as i32,
            bottom: self.rect_pos.bottom as i32,
        }
    }

    pub fn text_area(&self, is_active: bool) -> TextArea {
        let text_width = self.get_text_width();
        let TextWidth {
            width,
            buffer_width,
        } = text_width;

        let text_overlap = if width > buffer_width {
            width - buffer_width
        } else {
            0.0
        };

        TextArea {
            buffer: &self.buffer,
            left: self.rect_pos.left as f32 - text_overlap,
            top: self.top(),
            scale: 1.0,
            bounds: self.bounds(),
            default_color: if is_active {
                self.color_active
            } else {
                self.color
            },
        }
    }
}
