use super::{colors, DrawInfo, Event, Outcome, Widget};
use crate::gui::traits::DefaultWidgetDraw;
use crate::model::{EditorStateRef, OpenFile};
use glutin::event::VirtualKeyCode;
use skia_safe as skia;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct File {
    pub name: String,
    pub path: PathBuf,
    pub children: Vec<File>,
}

impl File {
    fn file(name: String, path: PathBuf) -> File {
        File {
            name,
            path,
            children: Vec::new(),
        }
    }

    fn undetermined(name: String, path: PathBuf) -> File {
        Self::file(name, path)
    }

    fn directory(name: String, path: PathBuf, children: Vec<File>) -> File {
        File {
            name,
            path,
            children,
        }
    }
}

#[derive(Default)]
struct Data {
    root: Option<File>,
    is_loading: bool,
    selected_index: usize,
}

pub struct Files {
    data: Arc<Mutex<Data>>,
    font: Rc<skia::Font>,
    editor_state: EditorStateRef,
    last_path: Option<PathBuf>,
}

impl Files {
    pub fn new(editor_state: EditorStateRef, font: Rc<skia::Font>) -> Files {
        Files {
            data: Default::default(),
            editor_state,
            font,
            last_path: None,
        }
    }

    pub fn load_files(&mut self, path: PathBuf) {
        self.data.lock().unwrap().is_loading = true;
        let data = self.data.clone();
        self.last_path = Some(path.clone());

        thread::spawn(move || {
            let files = read_files(path, 10);

            let mut d = data.lock().unwrap();
            d.root = Some(File::directory(".".into(), ".".into(), files));
            d.is_loading = false;
        });
    }

    pub fn reload_files(&mut self) {
        match self.last_path.clone() {
            Some(path) => self.load_files(path),
            _ => eprintln!("Can't reload because path was never set"),
        }
    }
}

fn read_files(path: PathBuf, depth_left: usize) -> Vec<File> {
    if depth_left == 0 {
        return Vec::new();
    }

    let mut result: Vec<File> = Vec::new();

    match fs::read_dir(&path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let name = entry
                            .file_name()
                            .into_string()
                            .unwrap_or_else(|_| String::new());

                        // TODO: change
                        if name == ".git" || name == "target" {
                            continue;
                        }

                        let file = match entry.metadata() {
                            Ok(md) => {
                                if md.is_dir() {
                                    File::directory(
                                        name,
                                        entry.path(),
                                        read_files(entry.path(), depth_left - 1),
                                    )
                                } else {
                                    File::file(name, entry.path())
                                }
                            }
                            Err(err) => {
                                eprintln!("{:?}", err);
                                File::undetermined(name, entry.path())
                            }
                        };

                        result.push(file);
                    }
                    Err(err) => eprintln!("{:?}", err),
                }
            }
        }
        _ => (),
    }
    result
}

impl Widget for Files {
    fn draw(&mut self, canvas: &mut skia::Canvas, bounds: &skia::Rect, info: DrawInfo) {
        canvas.save();
        canvas.clip_rect(bounds, skia::ClipOp::Intersect, false);
        let paint = skia::Paint::new(colors::white(), None);

        let data = self.data.lock().unwrap();
        if data.is_loading {
            canvas.draw_str("loading", skia::Point::new(0.0, 20.0), &*self.font, &paint);
        } else {
            let mut y = 20.0;
            let mut idx = 0;

            match &data.root {
                Some(f) => draw_file(
                    canvas,
                    &*self.font,
                    f,
                    0.0,
                    &mut y,
                    &mut idx,
                    data.selected_index,
                ),
                None => (),
            }
        }

        canvas.default_widget_draw(bounds, info);
        canvas.restore();
    }

    fn handle_event(&mut self, event: &Event) -> Outcome {
        match event {
            Event::KeyboardInput(input) => {
                if input.virtual_keycode == Some(VirtualKeyCode::Up) {
                    let mut data = self.data.lock().unwrap();

                    data.selected_index = data.selected_index.checked_sub(1).unwrap_or(0);
                    Outcome::Handled
                } else if input.virtual_keycode == Some(VirtualKeyCode::Down) {
                    let mut data = self.data.lock().unwrap();

                    data.selected_index += 1;
                    Outcome::Handled
                } else if input.virtual_keycode == Some(VirtualKeyCode::Return) {
                    let data = self.data.lock().unwrap();
                    let path = data
                        .root
                        .as_ref()
                        .and_then(|f| find_path_at_index(f, data.selected_index));

                    match path {
                        Some(path) => {
                            self.editor_state.borrow_mut().set_open_file(Some(OpenFile::new(path)))
                        }
                        None => eprintln!("nothing selected"),
                    }

                    Outcome::Handled
                } else if input.virtual_keycode == Some(VirtualKeyCode::F5) {
                    self.reload_files();

                    Outcome::Handled
                } else {
                    Outcome::Ignored
                }
            }
            _ => Outcome::Ignored,
        }
    }
}

fn draw_file(
    canvas: &mut skia::Canvas,
    font: &skia::Font,
    f: &File,
    x: f32,
    y: &mut f32,
    idx: &mut usize,
    selected_idx: usize,
) {
    let paint = skia::Paint::new(
        if selected_idx == *idx {
            colors::blue400()
        } else {
            colors::white()
        },
        None,
    );

    canvas.draw_str(&*f.name, skia::Point::new(x, *y), font, &paint);
    *y += 20.0;
    *idx += 1;

    for child in &f.children {
        draw_file(canvas, font, child, x + 20.0, y, idx, selected_idx);
    }
}

fn find_path_at_index(file: &File, index: usize) -> Option<PathBuf> {
    let mut res = None;
    let mut current_index = 0;

    find_path_at_index_rec(file, index, &mut current_index, &mut res);

    res
}

fn find_path_at_index_rec(
    file: &File,
    index: usize,
    current_index: &mut usize,
    res: &mut Option<PathBuf>,
) {
    if index == *current_index {
        *res = Some(file.path.clone());
    }

    *current_index += 1;
    for child in &file.children {
        find_path_at_index_rec(child, index, current_index, res);
    }
}
