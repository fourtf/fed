use super::text_model::{Selection, TextModel};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::PathBuf;
use std::collections::VecDeque;
use std::time::Instant;

#[derive(Default)]
pub struct OpenFile {
    pub model: TextModel,
    pub undo_stack: VecDeque<TextModel>,
    pub redo_stack: VecDeque<TextModel>,
    pub path: PathBuf,
    pub selection: Selection,
    pub last_edit_time: Option<Instant>,
}

impl OpenFile {
    // TODO: rename to load_from_path
    pub fn new(path: PathBuf) -> Self {
        let doc = TextModel::new();
        let doc = match doc.load_from(&path) {
            Err(e) => { println!("Error loading file: {}", e); doc },
            Ok(doc) => doc,
        };

        Self {
            model: doc,
            path,
            ..Default::default()
        }
    }

    pub fn save(&self) -> io::Result<()> {
        let file = File::create(&self.path)?;
        let mut file = BufWriter::new(file);

        for line in &self.model.lines {
            file.write(line.as_bytes())?;
            file.write("\n".as_bytes())?;
        }

        Ok(())
    }
}
