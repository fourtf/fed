use std::fmt;
use crate::model::LocDelta;
use smallvec::{SmallVec, smallvec};
use std::collections::HashMap;
use once_cell::sync::Lazy;

#[derive(Debug, PartialEq, Eq)]
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
    Cut,
    Paste,
    BeginSelection,
    EndSelection,
    GoTo(Location),
}

pub type EditorActions = SmallVec<[EditorAction; 4]>;

#[derive(Debug, Clone)]
pub enum Location {
    StartOfLine,
    EndOfLine,
    FirstLine,
    LastLine,
    Offset(LocDelta),
}

#[derive(Debug)]
enum KeybindAction {
    PerformEditorActions(EditorActions),
    EnterInsertMode,
    EnterVisualMode,
}

impl From<EditorAction> for KeybindAction {
    fn from(action: EditorAction) -> KeybindAction {
        return KeybindAction::PerformEditorActions(smallvec![action]);
    }
}

impl From<EditorActions> for KeybindAction {
    fn from(actions: EditorActions) -> KeybindAction {
        return KeybindAction::PerformEditorActions(actions);
    }
}

impl VimInput {
    pub fn new() -> Self {
        Self {
            mode: Mode::Normal,
        }
    }

    pub fn receive_char(&mut self, c: char) -> EditorActions {
        use EditorAction::*;

        match &self.mode {
            Mode::Normal => {
                match NORMAL_KEYBINDINGS.get(c.to_string().as_str()) {
                    Some(action) => self.perform_action(action),
                    None => Default::default(),
                }
            },
            Mode::Insert => {
                if c == '\r' {
                    smallvec![InsertNewline]
                } else if c == '\x08' {
                    smallvec![DeleteLeft]
                } else if c == '\x1B' {
                    self.mode = Mode::Normal;
                    smallvec![]
                } else if c >= ' ' {
                    smallvec![InsertString(c.to_string())]
                } else {
                    smallvec![]
                }
            }
            Mode::Visual => {
                if c == '\x1B' {
                    self.mode = Mode::Normal;
                    return smallvec![EditorAction::EndSelection];
                }
                
                return match VISUAL_KEYBINDINGS.get(c.to_string().as_str()) {
                    Some(action) => {
                        self.mode = Mode::Normal;
                        self.perform_action(action)
                    },
                    None => smallvec![],
                }
            },
        }
    }

    pub fn perform_action(&mut self, action: &KeybindAction) -> EditorActions {
        use KeybindAction::*;

        match action {
            EnterInsertMode => {
                self.mode = Mode::Insert;
                smallvec![]
            },
            EnterVisualMode => {
                self.mode = Mode::Visual;
                smallvec![EditorAction::BeginSelection]
            },
            PerformEditorActions(actions) => actions.clone(),
        }
    }
}

//macro_rules! ea {
//   ($x:expr) => (KeybindAction::PerformEditorActions($x))
//}

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

static NORMAL_KEYBINDINGS: Lazy<HashMap<&'static str, KeybindAction>> = Lazy::new(|| map!{
        "i" => KeybindAction::EnterInsertMode.into(),
        "a" => KeybindAction::EnterInsertMode.into(),
        "$" => EditorAction::GoTo(Location::EndOfLine).into(),
        "0" => EditorAction::GoTo(Location::StartOfLine).into(),
        "g" => EditorAction::GoTo(Location::FirstLine).into(),
        "G" => EditorAction::GoTo(Location::LastLine).into(),
        "v" => KeybindAction::EnterVisualMode.into(),
        "p" => EditorAction::Paste.into()
    });

static VISUAL_KEYBINDINGS: Lazy<HashMap<&'static str, KeybindAction>> = Lazy::new(|| map!{
        "y" => EditorAction::Copy.into(),
        "d" => EditorAction::Cut.into()
    });
