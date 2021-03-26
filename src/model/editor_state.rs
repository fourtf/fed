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

    pub fn set_open_file(&mut self, file: Option<OpenFile>) {
        self.file = file;

        //self.lsp_client.
    }

    pub fn open_file(&self) -> &Option<OpenFile> {
        return &self.open_file;
    }

    pub fn open_file_mut(&mut self) -> &mut Option<OpenFile> {
        return &mut self.open_file;
    }

    pub fn work_dir(&self) -> &PathBuf {
        return &self.work_dir;
    }

    pub fn set_lsp_client(&mut self, client: Option<lsp::Client>) {
        self.lsp_client = client;
    }

    pub fn input(&self) -> &VimInput {
        &mut self.input
    }

    pub fn input_mut(&self) -> &mut VimInput {
        &mut self.input
    }
}
