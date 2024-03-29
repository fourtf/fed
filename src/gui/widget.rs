use skia_safe as skia;

#[derive(Debug)]
pub enum Event {
    Input(char),
    KeyboardInput(glutin::event::KeyboardInput),
}

pub enum Outcome {
    Handled,
    Ignored,
}

#[derive(Debug, Clone, Copy)]
pub struct DrawInfo {
    pub is_focused: bool,
}

pub trait Widget {
    fn draw(&mut self, _canvas: &mut skia::Canvas, _bounds: &skia::Rect, _info: DrawInfo) {}

    fn handle_event(&mut self, _event: &Event) -> Outcome {
        Outcome::Ignored
    }
}
