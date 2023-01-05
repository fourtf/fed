use super::open_file::OpenFile;
use crate::input::VimInput;
use crate::lsp;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

pub struct EditorState {
    open_file: Option<OpenFile>,
    input: VimInput,
    work_dir: PathBuf,
    lsp_client: Option<lsp::Client>,
}

pub type EditorStateRef = Rc<RefCell<EditorState>>;

impl EditorState {
    pub fn new_ref(work_dir: PathBuf) -> EditorStateRef {
        Rc::new(RefCell::new(EditorState {
            open_file: None,
            input: VimInput::new(),
            work_dir,
            lsp_client: None,
        }))
    }

    pub fn open_file(&self) -> &Option<OpenFile> {
        return &self.open_file;
    }

    pub fn set_open_file(&mut self, open_file: Option<OpenFile>) {
        self.open_file = open_file;
    }

    pub fn edit_open_file(&mut self, f: impl FnOnce(&mut OpenFile)) {
        if let Some(ref mut open_file) = self.open_file {
            f(open_file);
        }
    }

    pub fn work_dir(&self) -> &PathBuf {
        return &self.work_dir;
    }

    pub fn set_lsp_client(&mut self, client: Option<lsp::Client>) {
        self.lsp_client = client;
    }

    pub fn input(&self) -> &VimInput {
        &self.input
    }

    pub fn input_mut(&mut self) -> &mut VimInput {
        &mut self.input
    }
}
