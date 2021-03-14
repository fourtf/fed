use super::open_file::OpenFile;
use std::cell::RefCell;
use std::rc::Rc;
use crate::input::VimInput;

pub struct EditorState {
    pub open_file: OpenFile,
    pub input: VimInput,
}

pub type EditorStateRef = Rc<RefCell<EditorState>>;

pub fn make_ref(state: EditorState) -> EditorStateRef {
    Rc::new(RefCell::new(state))
}
