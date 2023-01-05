use super::traits::DefaultWidgetDraw;
use super::widget::{DrawInfo, Event, Outcome, Widget};
use crate::input::Location;
use crate::model::{EditorState, EditorStateRef, OpenFile, Selection, TextModel};
use clipboard::{ClipboardContext, ClipboardProvider};
use glutin::event::{ModifiersState, VirtualKeyCode};
use skia_safe as skia;
use tap::prelude::*;

use std::rc::Rc;
use std::time::Instant;

pub struct Editor {
    pub state: EditorStateRef,
    pub font: Rc<skia::Font>,
}

impl Widget for Editor {
    fn draw(&mut self, canvas: &mut skia::Canvas, bounds: &skia::Rect, info: DrawInfo) {
        let state = self.state.borrow();

        let font = &*self.font;
        let empty_file: OpenFile = Default::default();
        let open_file = state.open_file().as_ref().unwrap_or(&empty_file);
        let doc = &open_file.model;
        let selection = &open_file.selection;
        let input = &state.input();

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
        canvas.default_widget_draw(bounds, info);
    }

    fn handle_event(&mut self, event: &Event) -> Outcome {
        use crate::input::EditorAction::*;
        let mut state = self.state.borrow_mut();

        let transform_text = |state: &mut EditorState, f: &dyn Fn(&mut OpenFile) -> TextModel| {
            state.edit_open_file(|file| {
                f(file);

                // map
                file.model = f(file);

                // undo
                if file
                    .last_edit_time
                    .map(|x| x.elapsed().as_secs() < 1)
                    .unwrap_or(false)
                {
                    file.undo_stack.pop_front();
                }
                file.undo_stack.push_front(file.model.clone());

                if file.undo_stack.len() > 100 {
                    file.undo_stack.pop_back();
                }
                file.redo_stack.clear();

                // timestamp
                file.last_edit_time = Some(Instant::now());
            });
        };

        match event {
            Event::Input(c) => {
                let actions = state.input_mut().receive_char(*c);

                match state.open_file() {
                    Some(open_file) => (),
                    None => return Outcome::Ignored,
                };

                for action in actions {
                    match action {
                        InsertString(str) => {
                            transform_text(&mut state, &|x| x.model.insert(str.as_str()))
                        }
                        InsertNewline => transform_text(&mut state, &|x| x.model.insert_newline()),
                        DeleteLeft => transform_text(&mut state, &|x| x.model.backspace_key()),

                        Copy => {
                            let transform = |file: &mut OpenFile| -> TextModel {
                                let s = file.model.get_string(file.selection);
                                let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                                ctx.set_contents(s).unwrap();
                                file.model.clone()
                            };
                            transform_text(&mut state, &transform);
                        }
                        Paste => {
                            let transform = |file: &mut OpenFile| -> TextModel {
                                let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                                let s = ctx.get_contents().unwrap_or(String::new());
                                file.model.insert(&*s)
                            };
                            transform_text(&mut state, &transform);
                        }
                        Cut => {
                            let transform = |file: &mut OpenFile| -> TextModel {
                                let selection = file.selection;
                                file.selection = Selection::empty();
                                let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                                ctx.set_contents(file.model.get_string(selection)).unwrap();
                                file.model.delete(selection)
                            };
                            transform_text(&mut state, &transform);
                        }
                        BeginSelection => {
                            let transform = |file: &mut OpenFile| -> TextModel {
                                let cursor = file.model.cursor;
                                file.selection = crate::model::Selection::new(cursor, cursor);
                                file.model.clone()
                            };
                            transform_text(&mut state, &transform);
                        }
                        EndSelection => {
                            let transform = |file: &mut OpenFile| -> TextModel {
                                file.selection = Selection::empty();
                                file.model.clone()
                            };
                            transform_text(&mut state, &transform);
                        }
                        GoTo(loc) => {
                            let transform = |file: &mut OpenFile| -> TextModel {
                                match loc {
                                    Location::EndOfLine => file.model.move_cursor((0, 1000000000)),
                                    Location::StartOfLine => {
                                        file.model.move_cursor((0, -1000000000))
                                    }
                                    Location::FirstLine => file.model.move_cursor((-1000000000, 0)),
                                    Location::LastLine => file.model.move_cursor((1000000000, 0)),
                                    Location::Offset(delta) => file.model.move_cursor(delta),
                                }
                            };
                            transform_text(&mut state, &transform);
                        }
                        Undo => {
                            let transform = |file: &mut OpenFile| -> TextModel {
                                let old_model = file.model.clone();
                                match file.undo_stack.pop_front() {
                                    Some(model) => {
                                        file.redo_stack.push_front(old_model);
                                        model
                                    }
                                    None => file.model.clone(),
                                }
                            };
                            transform_text(&mut state, &transform);
                        }
                        Redo => {
                            let transform = |file: &mut OpenFile| -> TextModel {
                                let old_model = file.model.clone();
                                match file.redo_stack.pop_front() {
                                    Some(model) => {
                                        file.undo_stack.push_front(old_model);
                                        model
                                    }
                                    None => file.model.clone(),
                                }
                            };
                            transform_text(&mut state, &transform);
                        }
                        FormatDocument => {
                            let transform = |file: &mut OpenFile| -> TextModel {
                                match rust_fmt(&file.model.get_string_all()) {
                                    Ok(text) => {
                                        println!("Formatted");
                                        file.model.with_content(&*text)
                                    }
                                    Err(e) => {
                                        eprintln!("error while formatting: {}", e);
                                        file.model.clone()
                                    }
                                }
                            };
                            transform_text(&mut state, &transform);
                        }
                    };
                }

                Outcome::Handled
            }

            #[allow(deprecated)]
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
                        let transform =
                            |file: &mut OpenFile| -> TextModel { file.model.move_cursor((-1, 0)) };
                        transform_text(&mut state, &transform);
                    } else if Some(VirtualKeyCode::Down) == virtual_keycode {
                        let transform =
                            |file: &mut OpenFile| -> TextModel { file.model.move_cursor((1, 0)) };
                        transform_text(&mut state, &transform);
                    } else if Some(VirtualKeyCode::Left) == virtual_keycode {
                        let transform =
                            |file: &mut OpenFile| -> TextModel { file.model.move_cursor((0, -1)) };
                        transform_text(&mut state, &transform);
                    } else if Some(VirtualKeyCode::Right) == virtual_keycode {
                        let transform =
                            |file: &mut OpenFile| -> TextModel { file.model.move_cursor((0, 1)) };
                        transform_text(&mut state, &transform);
                    } else if Some(VirtualKeyCode::Tab) == virtual_keycode {
                        let transform =
                            |file: &mut OpenFile| -> TextModel { file.model.insert("    ") };
                        transform_text(&mut state, &transform);
                    } else {
                        outcome = Outcome::Ignored;
                    }

                    let mode = state.input().mode;

                    state.edit_open_file(|open_file| match mode {
                        crate::input::Mode::Visual => {
                            let selection = open_file.selection;
                            let cursor = open_file.model.cursor;
                            open_file.selection = selection.with_end(cursor);
                        }
                        crate::input::Mode::VisualLine => {
                            let selection = open_file.selection;
                            let cursor = open_file.model.cursor;
                            let lines = open_file.model.lines.clone();
                            let mut selection = selection.with_end(cursor);
                            selection.first.column = 0;
                            selection.last.column =
                                lines.get(selection.last.row).map(|x| x.len()).unwrap_or(0);
                            open_file.selection = selection;
                        }
                        _ => (),
                    });

                    outcome
                } else if modifiers == ModifiersState::CTRL {
                    if Some(VirtualKeyCode::S) == virtual_keycode {
                        state
                            .open_file()
                            .as_ref()
                            .tap_some(|open_file| match open_file.save() {
                                Err(e) => println!("Error while saving: {}", e),
                                _ => println!("Saved successfully"),
                            });
                        Outcome::Handled
                    } else {
                        Outcome::Ignored
                    }
                } else {
                    Outcome::Ignored
                }
            }
        }
    }
}

fn rust_fmt(s: &String) -> std::io::Result<String> {
    use std::io::Write;
    use std::process::*;

    let mut child = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let child_stdin = child.stdin.as_mut().unwrap();
    child_stdin.write_all(s.as_bytes())?;
    drop(child_stdin);

    let output = child.wait_with_output()?;

    Ok(String::from_utf8_lossy(&*output.stdout).into())
}
