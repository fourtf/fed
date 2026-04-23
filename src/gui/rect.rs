use super::Shrink1;
use super::widget::{DrawInfo, Widget};
use crate::gui::traits::DefaultWidgetDraw;
use skia_safe as skia;

#[allow(unused)]
pub struct Rect {}

impl Widget for Rect {
    fn draw(&mut self, canvas: &skia::Canvas, bounds: &skia::Rect, info: DrawInfo) {
        let mut paint = skia::Paint::new(skia::Color4f::new(1.0, 0.0, 0.0, 1.0), None);
        paint.set_style(skia::paint::Style::Stroke);

        canvas.draw_rect(&bounds.shrunken_by_1(), &paint);

        canvas.default_widget_draw(bounds, info);
    }
}
