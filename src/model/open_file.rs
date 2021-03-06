use super::text_model::{Selection, TextModel};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::PathBuf;

pub struct OpenFile {
    pub model: TextModel,
    pub path: PathBuf,
    pub selection: Selection,
}

impl OpenFile {
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
