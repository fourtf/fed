use std::fmt;
use phf::phf_map;

#[derive(Debug)]
pub enum Mode {
    Normal,
    Insert,
    Visual,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct VimInput {
    pub mode: Mode,
}

#[derive(Debug, Clone)]
pub enum EditorAction {
    InsertString(String),
    DeleteLeft,
    InsertNewline,
    Copy,
    CopyLine,
    Paste,
}

#[derive(Debug)]
enum KeybindAction {
    PerformEditorAction(EditorAction),
    EnterInsertMode,
    EnterVisualMode,
}

impl From<EditorAction> for KeybindAction {
    fn from(action: EditorAction) -> KeybindAction {
        return KeybindAction::PerformEditorAction(action);
    }
}

impl VimInput {
    pub fn new() -> Self {
        Self {
            mode: Mode::Normal,
        }
    }

    pub fn receive_char(&mut self, c: char) -> Option<EditorAction> {
        use EditorAction::*;

        match &self.mode {
            Mode::Normal => {
                match NORMAL_KEYBINDINGS.get(c.to_string().as_str()) {
                    Some(action) => self.perform_action(action),
                    None => None
                }
            },
            Mode::Insert => {
                if c == '\r' {
                    Some(InsertNewline)
                } else if c == '\x08' {
                    Some(DeleteLeft)
                } else if c == '\x1B' {
                    self.mode = Mode::Normal;
                    None
                } else if c >= ' ' {
                    Some(InsertString(c.to_string()))
                } else {
                    None
                }
            }
            Mode::Visual => {
                if c == '\x1B' {
                    self.mode = Mode::Normal;
                }
                None
            },
        }
    }

    pub fn perform_action(&mut self, action: &KeybindAction) -> Option<EditorAction> {
        use KeybindAction::*;

        match action {
            EnterInsertMode => {
                self.mode = Mode::Insert;
                None
            },
            EnterVisualMode => {
                self.mode = Mode::Visual;
                None
            },
            PerformEditorAction(action) => Some(action.clone()), 
        }
    }
}

macro_rules! ea {
    ($x:expr) => (KeybindAction::PerformEditorAction($x))
}

static NORMAL_KEYBINDINGS: phf::Map<&'static str, KeybindAction> = phf_map! {
    "i" => KeybindAction::EnterInsertMode,
    "a" => KeybindAction::EnterInsertMode,
    "v" => KeybindAction::EnterVisualMode,
    "yy" => ea!(EditorAction::CopyLine),
    "p" => ea!(EditorAction::Paste),
};

static VISUAL_KEYBINDINGS: phf::Map<&'static str, KeybindAction> = phf_map! {
    "y" => KeybindAction::PerformEditorAction(EditorAction::Copy),
};
