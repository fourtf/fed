use skia_safe as skia;
use super::widget::{Widget, Outcome};
use super::Shrink1;

pub struct Rect {}

impl Widget for Rect {
    fn draw(&mut self, canvas: &mut skia::Canvas, bounds: &skia::Rect) {
        let mut paint = skia::Paint::new(skia::Color4f::new(1.0, 0.0, 0.0, 1.0), None);
        paint.set_style(skia::paint::Style::Stroke);

        canvas.draw_rect(
            &bounds.shrunken_by_1(),
            &paint);
    }
}
