use skia_safe as skia;
use skia_safe::{Canvas, Rect, Paint};
use super::DrawInfo;
use super::colors;

pub trait Shrink1 {
    fn shrunken_by_1(&self) -> Rect;
}

impl Shrink1 for Rect {
    fn shrunken_by_1(&self) -> Rect {
        Self::from_xywh(self.x(), self.y(), self.width() - 1.0, self.height() - 1.0)
    }
}


pub trait DefaultWidgetDraw {
    fn default_widget_draw(&mut self, bounds: &Rect, info: DrawInfo);
}

impl DefaultWidgetDraw for Canvas {
    fn default_widget_draw(&mut self, bounds: &Rect, info: DrawInfo) {
        if info.is_focused {
            let mut paint = Paint::new(colors::blue400(), None);
            paint.set_style(skia::paint::Style::Stroke);

            self.draw_rect(bounds, &paint);
        }
    }
}
