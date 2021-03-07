use skia_safe as skia;
use super::widget::{Event, Widget};
use crate::input::VimInput;
use crate::model::{Selection, TextModel};

pub struct Editor<'a> {
    pub model: &'a TextModel,
    pub selection: &'a Selection,

    pub font: &'a skia::Font,
    pub input: &'a VimInput    
}

impl<'a> Widget for Editor<'a> {
    fn draw(&mut self, canvas: &mut skia::Canvas, bounds: &skia::Rect) {
        let font = self.font;
        let doc = self.model;
        let selection = self.selection;
        let input = self.input;

        let paint = skia::Paint::new(skia::Color4f::new(1.0, 1.0, 1.0, 1.0), None);
        let bg_paint = skia::Paint::new(skia::Color4f::new(0.2, 0.2, 0.2, 1.0), None);
        let selection_paint = skia::Paint::new(skia::Color4f::new(0.2, 0.22, 0.3, 1.0), None);
        let linenr_paint = skia::Paint::new(skia::Color4f::new(0.6, 0.6, 0.6, 1.0), None);
        let mut cursor_paint = skia::Paint::new(skia::Color4f::new(1.0, 1.0, 1.0, 1.0), None);
        cursor_paint.set_style(skia::paint::Style::Stroke);

        let (_, rect) = font.measure_str("Xg", Some(&paint));
        let (x_width, _) = font.measure_str("X", Some(&paint));
        let line_height = rect.height() * 1.6;
        let text_offset_y = rect.height() * 1.3;

        let mut y = -line_height * (doc.cursor.row as f32) + 500.;

        let linenr_width = 5;
        let line_offset_x = x_width * (linenr_width + 1) as f32;

        for (rowi, line) in doc.lines.iter().enumerate() {
            // SELECTION
            if !selection.is_empty() &&
                    selection.first.row <= rowi &&
                    selection.last.row >= rowi {
                let l = if selection.first.row == rowi { x_width * (selection.first.column as f32) } else { 0. };
                let r = if selection.last.row == rowi { x_width * (selection.last.column as f32) } else { x_width * (line.len() as f32) };

                canvas.draw_rect(
                    skia::Rect::new(
                        line_offset_x + l,
                        y,
                        line_offset_x + r + (x_width * 0.2),
                        y + line_height
                    ),
                    &selection_paint);
            }

            // LINENR
            let mut linenr = [0u8; 20];
            let n = itoa::write(&mut linenr[..], rowi + 1).unwrap_or(0);
            canvas.draw_str(
                unsafe { std::str::from_utf8_unchecked(&linenr[..n]) },
                skia::Point::new(x_width * (linenr_width - n) as f32, y + text_offset_y),
                font,
                &linenr_paint);

            // LINE
            canvas.draw_str(
                line.as_str(),
                skia::Point::new(line_offset_x, y + text_offset_y),
                font,
                &paint,
            );

            // CURSOR
            if rowi == doc.cursor.row {
                let rect =
                    skia::Rect::from_xywh(
                        line_offset_x + x_width * (doc.cursor.column as f32),
                        y,
                        if input.mode == crate::input::Mode::Normal { x_width } else { 1. },
                        line_height);
                canvas.draw_rect(&rect, &cursor_paint);
            }

            y += line_height;
        }

        canvas.draw_rect(skia::Rect::from_xywh(0., 0., 10000., line_height), &bg_paint);

        canvas.draw_str(
            input.mode.to_string(),
            skia::Point::new(0., text_offset_y),
            font,
            &paint,
        );
    }
}
