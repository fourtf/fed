use super::widget::{Event, Outcome, Widget, DrawInfo};
use skia_safe as skia;

pub struct Container {
    pub items: Vec<Box<dyn Widget>>,
    pub selected_index: usize,
}

impl Container {
    pub fn make_row(items: Vec<Box<dyn Widget>>) -> Container {
        Container {
            items,
            selected_index: 0,
        }
    }
}

impl Container {
    #[allow(deprecated)]
    fn handle_own_event(&mut self, event: &Event) -> Option<Outcome> {
        use glutin::event::{ModifiersState, VirtualKeyCode};
        // handle alt+arrows
        match event {
            Event::KeyboardInput(input) => {
                if input.modifiers == ModifiersState::ALT
                    && input.virtual_keycode == Some(VirtualKeyCode::Left)
                {
                    self.offset_selected_index(-1)
                } else if input.modifiers == ModifiersState::ALT
                    && input.virtual_keycode == Some(VirtualKeyCode::Right)
                {
                    self.offset_selected_index(1)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn offset_selected_index(&mut self, offset: isize) -> Option<Outcome> {
        let old_index = self.selected_index;
        let new_index = ((self.selected_index as isize) + offset)
            .max(0)
            .min(self.items.len() as isize - 1) as usize;
        if old_index != new_index {
            self.selected_index = new_index;
            Some(Outcome::Handled)
        } else {
            None
        }
    }

    fn propagate_event(&mut self, event: &Event) -> Outcome {
        match self.items.get_mut(self.selected_index) {
            Some(item) => item.handle_event(event),
            _ => Outcome::Ignored,
        }
    }
}

impl Widget for Container {
    fn draw(&mut self, canvas: &mut skia::Canvas, bounds: &skia::Rect, info: DrawInfo) {
        let element_width = bounds.width() as f32 / self.items.len().max(1) as f32;
        let mut x = bounds.x();

        for (i, item) in &mut self.items.iter_mut().enumerate() {
            item.draw(
                canvas,
                &skia::Rect::from_xywh(x, bounds.y(), element_width, bounds.height()),
                DrawInfo {
                    is_focused: info.is_focused && i == self.selected_index,
                    ..info
                }
            );

            x += element_width;
        }
    }

    fn handle_event(&mut self, event: &Event) -> Outcome {
        self.handle_own_event(event)
            .unwrap_or_else(|| self.propagate_event(event))
    }
}

#[macro_export]
macro_rules! collection_items{
    ($($val:expr),*$(,)* ) => ({
        {
            let mut v: Vec<Box<dyn Widget>> = Vec::new();
            $(
                v.push($val);
            )+
            v
        }
     });
}
