use skia_safe as skia;
use super::widget::{Widget, DrawInfo};
use super::Shrink1;
use crate::gui::traits::DefaultWidgetDraw;

pub struct Rect {}

impl Widget for Rect {
    fn draw(&mut self, canvas: &mut skia::Canvas, bounds: &skia::Rect, info: DrawInfo) {
        let mut paint = skia::Paint::new(skia::Color4f::new(1.0, 0.0, 0.0, 1.0), None);
        paint.set_style(skia::paint::Style::Stroke);

        canvas.draw_rect(
            &bounds.shrunken_by_1(),
            &paint);

        canvas.default_widget_draw(bounds, info);
    }
}
