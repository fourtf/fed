use std::fmt;
use crate::model::LocDelta;
use smallvec::{SmallVec, smallvec};
use std::collections::HashMap;
use once_cell::sync::Lazy;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Mode {
    Normal,
    Insert,
    Visual,
    VisualLine,
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
    Undo,
    Redo,
}

impl EditorAction {
    fn k(self) -> KeybindAction {
        KeybindAction::PerformEditorActions(smallvec![self])
    }

    fn ks(self) -> KeybindActions {
        smallvec![self.k()]
    }
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
    EnterMode(Mode),
}

type KeybindActions = SmallVec<[KeybindAction; 4]>;

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
                    Some(action) => self.perform_actions(action),
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
            Mode::Visual | Mode::VisualLine => {
                if c == '\x1B' {
                    self.mode = Mode::Normal;
                    return smallvec![EditorAction::EndSelection];
                }
                
                return match VISUAL_KEYBINDINGS.get(c.to_string().as_str()) {
                    Some(actions) => {
                        self.mode = Mode::Normal;
                        self.perform_actions(actions)
                    },
                    None => smallvec![],
                }
            },
        }
    }

    fn perform_actions(&mut self, actions: &KeybindActions) -> EditorActions {
        use KeybindAction::*;

        actions.iter().map(|action|
            match action {
                EnterMode(mode) => {
                    self.mode = *mode;
                    smallvec![]
                },
                PerformEditorActions(actions) => actions.clone(),
            }
        ).flatten().collect()
    }
}

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

use EditorAction as E;
use KeybindAction as K;

static NORMAL_KEYBINDINGS: Lazy<HashMap<&'static str, KeybindActions>> = Lazy::new(|| map!{
        "i" => smallvec![ K::EnterMode(Mode::Insert) ],
        "a" => smallvec![ E::GoTo(Location::Offset((0, 1).into())).k(), K::EnterMode(Mode::Insert) ],
        "$" => E::GoTo(Location::EndOfLine).ks(),
        "0" => E::GoTo(Location::StartOfLine).ks(),
        "A" => smallvec![ E::GoTo(Location::EndOfLine).k(), K::EnterMode(Mode::Insert) ],
        "I" => smallvec![ E::GoTo(Location::StartOfLine).k(), K::EnterMode(Mode::Insert) ],
        "g" => E::GoTo(Location::FirstLine).ks(),
        "G" => E::GoTo(Location::LastLine).ks(),
        "v" => smallvec![ K::EnterMode(Mode::Visual), E::BeginSelection.k() ],
        "V" => smallvec![ K::EnterMode(Mode::VisualLine), E::BeginSelection.k() ],
        "p" => E::Paste.ks(),
        "u" => E::Undo.ks(),
        "U" => E::Redo.ks(),
        "O" => smallvec![ E::GoTo(Location::StartOfLine).k(), E::InsertNewline.k(),
                    E::GoTo(Location::Offset((-1, 0).into())).k(), K::EnterMode(Mode::Insert) ],
        "o" => smallvec![ E::GoTo(Location::EndOfLine).k(), E::InsertNewline.k(), K::EnterMode(Mode::Insert) ]
    });

static VISUAL_KEYBINDINGS: Lazy<HashMap<&'static str, KeybindActions>> = Lazy::new(|| map!{
        "y" => smallvec![ E::Copy.k(), E::EndSelection.k() ],
        "d" => smallvec![ E::Copy.k(), E::Cut.k(), E::EndSelection.k() ],
        "c" => smallvec![ E::Copy.k(), E::Cut.k(), E::EndSelection.k(), K::EnterMode(Mode::Insert) ]
    });
