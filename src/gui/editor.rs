use super::widget::{Event, Widget, Outcome};
use crate::input::{Location, VimInput};
use crate::model::EditorStateRef;
use crate::model::{Selection, TextModel};
use clipboard::{ClipboardContext, ClipboardProvider};
use glutin::event::{ElementState, KeyboardInput, ModifiersState, VirtualKeyCode, WindowEvent};
use skia_safe as skia;
use std::convert::TryInto;
use std::rc::Rc;
use std::time::Instant;

pub struct Editor {
    pub state: EditorStateRef,
    pub font: Rc<skia::Font>,
}

impl Widget for Editor {
    fn draw(&mut self, canvas: &mut skia::Canvas, bounds: &skia::Rect) {
        let state = self.state.borrow_mut();

        let font = &*self.font;
        let doc = &state.open_file.model;
        let selection = &state.open_file.selection;
        let input = &state.input;

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

        canvas.save();
        canvas.clip_rect(bounds, skia::ClipOp::Intersect, false);
        canvas.translate((bounds.x(), bounds.y()));

        let mut y = -line_height * (doc.cursor.row as f32) + 500.;

        let linenr_width = 5;
        let line_offset_x = x_width * (linenr_width + 1) as f32;

        for (rowi, line) in doc.lines.iter().enumerate() {
            // SELECTION
            if !selection.is_empty() && selection.first.row <= rowi && selection.last.row >= rowi {
                let l = if selection.first.row == rowi {
                    x_width * (selection.first.column as f32)
                } else {
                    0.
                };
                let r = if selection.last.row == rowi {
                    x_width * (selection.last.column as f32)
                } else {
                    x_width * (line.len() as f32)
                };

                canvas.draw_rect(
                    skia::Rect::new(
                        line_offset_x + l,
                        y,
                        line_offset_x + r + (x_width * 0.2),
                        y + line_height,
                    ),
                    &selection_paint,
                );
            }

            // LINENR
            let mut linenr = [0u8; 20];
            let n = itoa::write(&mut linenr[..], rowi + 1).unwrap_or(0);
            canvas.draw_str(
                unsafe { std::str::from_utf8_unchecked(&linenr[..n]) },
                skia::Point::new(x_width * (linenr_width - n) as f32, y + text_offset_y),
                font,
                &linenr_paint,
            );

            // LINE
            canvas.draw_str(
                line.as_str(),
                skia::Point::new(line_offset_x, y + text_offset_y),
                font,
                &paint,
            );

            // CURSOR
            if rowi == doc.cursor.row {
                let rect = skia::Rect::from_xywh(
                    line_offset_x + x_width * (doc.cursor.column as f32),
                    y,
                    if input.mode == crate::input::Mode::Normal {
                        x_width
                    } else {
                        1.
                    },
                    line_height,
                );
                canvas.draw_rect(&rect, &cursor_paint);
            }

            y += line_height;
        }

        canvas.draw_rect(
            skia::Rect::from_xywh(0., 0., 10000., line_height),
            &bg_paint,
        );

        canvas.draw_str(
            input.mode.to_string(),
            skia::Point::new(0., text_offset_y),
            font,
            &paint,
        );

        canvas.restore();
    }

    fn handle_event(&mut self, event: &Event) -> Outcome {
        use crate::input::EditorAction::*;
        let mut state = &self.state;

        let map_text_model = |f: &dyn Fn(TextModel) -> TextModel| {
            let new = state.borrow().open_file.model.clone();

            // map
            let new = f(new);
            let mut b = state.borrow_mut();
            b.open_file.model = new.clone();

            // undo
            if b.open_file
                .last_edit_time
                .map(|x| x.elapsed().as_secs() < 1)
                .unwrap_or(false)
            {
                b.open_file.undo_stack.pop_front();
            }
            b.open_file.undo_stack.push_front(new);

            if b.open_file.undo_stack.len() > 100 {
                b.open_file.undo_stack.pop_back();
            }
            b.open_file.redo_stack.clear();

            b.open_file.last_edit_time = Some(Instant::now());
        };

        match event {
            Event::Input(c) => {
                let actions = state.borrow_mut().input.receive_char(*c);
                for action in actions {
                    match action {
                        InsertString(str) => map_text_model(&|x| x.insert(str.as_str())),
                        InsertNewline => map_text_model(&|x| x.insert_newline()),
                        DeleteLeft => map_text_model(&|x| x.backspace_key()),
                        Copy => {
                            let s = state
                                .borrow()
                                .open_file
                                .model
                                .get_string(state.borrow().open_file.selection);

                            // TODO: make safe
                            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                            ctx.set_contents(s).unwrap();
                        }
                        Paste => {
                            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                            // TODO: make safe
                            let s = ctx.get_contents().unwrap_or(String::new());
                            map_text_model(&|x| x.insert(&*s));
                        }
                        Cut => {
                            let selection = state.borrow().open_file.selection;
                            state.borrow_mut().open_file.selection = Selection::empty();
                            map_text_model(&|x| x.delete(selection));
                        }
                        BeginSelection => {
                            let cursor = state.borrow().open_file.model.cursor;
                            state.borrow_mut().open_file.selection =
                                crate::model::Selection::new(cursor, cursor);
                        }
                        EndSelection => {
                            state.borrow_mut().open_file.selection = Selection::empty();
                        }
                        GoTo(loc) => match loc {
                            Location::EndOfLine => {
                                map_text_model(&|x| x.move_cursor((0, 1000000000)))
                            }
                            Location::StartOfLine => {
                                map_text_model(&|x| x.move_cursor((0, -1000000000)))
                            }
                            Location::FirstLine => {
                                map_text_model(&|x| x.move_cursor((-1000000000, 0)))
                            }
                            Location::LastLine => {
                                map_text_model(&|x| x.move_cursor((1000000000, 0)))
                            }
                            Location::Offset(delta) => map_text_model(&|x| x.move_cursor(delta)),
                        },
                        Undo => {
                            let old_model = state.borrow().open_file.model.clone();
                            let mut s = state.borrow_mut();
                            match s.open_file.undo_stack.pop_front() {
                                Some(model) => {
                                    s.open_file.redo_stack.push_front(old_model);
                                    s.open_file.model = model;
                                }
                                None => (),
                            }
                        }
                        Redo => {
                            let old_model = state.borrow().open_file.model.clone();
                            let mut s = state.borrow_mut();
                            match s.open_file.redo_stack.pop_front() {
                                Some(model) => {
                                    s.open_file.undo_stack.push_front(old_model);
                                    s.open_file.model = model;
                                }
                                None => (),
                            }
                        }
                    };
                }

                Outcome::Handled
            }

            Event::KeyboardInput(glutin::event::KeyboardInput {
                virtual_keycode,
                modifiers,
                ..
            }) => {
                let virtual_keycode = *virtual_keycode;
                let modifiers = *modifiers;

                if modifiers == ModifiersState::empty() {
                    let mut outcome = Outcome::Handled;

                    if Some(VirtualKeyCode::Up) == virtual_keycode {
                        map_text_model(&|x| x.move_cursor((-1, 0)));
                    } else if Some(VirtualKeyCode::Down) == virtual_keycode {
                        map_text_model(&|x| x.move_cursor((1, 0)));
                    } else if Some(VirtualKeyCode::Left) == virtual_keycode {
                        map_text_model(&|x| x.move_cursor((0, -1)));
                    } else if Some(VirtualKeyCode::Right) == virtual_keycode {
                        map_text_model(&|x| x.move_cursor((0, 1)));
                    } else if Some(VirtualKeyCode::Tab) == virtual_keycode {
                        map_text_model(&|x| x.insert("    "));
                    } else {
                        outcome = Outcome::Ignored;
                    }

                    let mode = state.borrow().input.mode;
                    match mode {
                        crate::input::Mode::Visual => {
                            let selection = state.borrow().open_file.selection;
                            let cursor = state.borrow().open_file.model.cursor;
                            state.borrow_mut().open_file.selection = selection.with_end(cursor);
                        }
                        crate::input::Mode::VisualLine => {
                            let selection = state.borrow().open_file.selection;
                            let cursor = state.borrow().open_file.model.cursor;
                            let lines = state.borrow().open_file.model.lines.clone();
                            let mut selection = selection.with_end(cursor);
                            selection.first.column = 0;
                            selection.last.column =
                                lines.get(selection.last.row).map(|x| x.len()).unwrap_or(0);
                            state.borrow_mut().open_file.selection = selection;
                        }
                        _ => (),
                    }

                    outcome
                } else if modifiers == ModifiersState::CTRL {
                    if Some(VirtualKeyCode::S) == virtual_keycode {
                        match state.borrow().open_file.save() {
                            Err(e) => println!("Error while saving: {}", e),
                            _ => println!("Saved successfully"),
                        }
                        Outcome::Handled
                    } else {
                        Outcome::Ignored
                    }
                } else {
                    Outcome::Ignored
                }
            }
            _ => Outcome::Ignored,
        }
    }
}
