use skia_safe as skia;

#[derive(Debug)]
pub enum Event {
    Input(char),
}

pub trait Widget {
    fn draw(&mut self, canvas: &mut skia::Canvas, bounds: &skia::Rect) {
        let mut paint = skia::Paint::new(skia::Color4f::new(1.0, 0.0, 0.0, 1.0), None);
        paint.set_style(skia::paint::Style::Stroke);

        canvas.draw_rect(bounds, &paint);
    }

    fn handle_event(&mut self, event: &Event) {
        println!("draw {:?}", event);
    }
}
