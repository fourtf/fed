use super::text_model::TextModel;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::LineWriter;
use std::path::PathBuf;

pub struct OpenFile {
    pub model: TextModel,
    pub path: PathBuf,
}

impl OpenFile {
    pub fn save(&self) -> io::Result<()> {
        let file = File::create(&self.path)?;
        let mut file = LineWriter::new(file);

        for line in &self.model.lines {
            file.write(line.as_bytes())?;
        }

        Ok(())
    }
}
