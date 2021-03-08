use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

/**
 * Location in text.
 */
#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Copy, Clone, Debug)]
pub struct Loc {
    pub row: usize,
    pub column: usize,
}

impl Loc {
    pub fn new(row: usize, column: usize) -> Loc {
        Loc { row, column }
    }

    pub fn zero() -> Loc {
        Loc { row: 0, column: 0 }
    }

    pub fn offset<D: Into<LocDelta>>(&self, offset: D) -> Loc {
        let offset = offset.into();

        Loc {
            row: ((self.row as isize) + offset.row).max(0) as usize,
            column: ((self.column as isize) + offset.column).max(0) as usize,
        }
    }

    /**
     * Examples:
     * - (10, 5) + (0, 5) = (10, 10)
     * - (10, 5) + (10, 7) = (20, *7*)
     */
    pub fn add_wrapped<D: Into<Loc>>(&self, offset: D) -> Loc {
        let offset = offset.into();
        if offset.row == 0 {
            Loc {
                row: self.row,
                column: self.column + offset.column,
            }
        } else {
            Loc {
                row: self.row + offset.row,
                column: offset.column,
            }
        }
    }

    pub fn constrain(&self, lines: &im::Vector<String>) -> Loc {
        let lc = get_last_cursor(lines);
        let cursor = self.min(&lc);

        let max_col = lines.get(cursor.row as usize).map(|x| x.len()).unwrap_or(0) as usize;
        Loc::new(cursor.row, cursor.column.min(max_col))
    }

    pub fn constrain_only_row(&self, lines: &im::Vector<String>) -> Loc {
        let lc = get_last_cursor(lines);
        self.min(&lc).clone()
    }

    pub fn with_row(&self, row: usize) -> Self {
        Self {
            column: self.column,
            row,
        }
    }

    pub fn with_column(&self, column: usize) -> Self {
        Self {
            column,
            row: self.row,
        }
    }
}

impl From<(usize, usize)> for Loc {
    fn from(tuple: (usize, usize)) -> Self {
        Loc::new(tuple.0, tuple.1)
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct LocDelta {
    row: isize,
    column: isize,
}

impl LocDelta {
    pub fn new(row: isize, column: isize) -> LocDelta {
        LocDelta { row, column }
    }
}

impl From<(isize, isize)> for LocDelta {
    fn from(tuple: (isize, isize)) -> Self {
        LocDelta::new(tuple.0, tuple.1)
    }
}

fn get_last_cursor(lines: &im::Vector<String>) -> Loc {
    Loc::new(
        ((lines.len() as isize) - 1).max(0) as usize,
        lines.last().map(|x| x.len()).unwrap_or(0) as usize,
    )
}

/**
 * start/end are the logical start and end of the selection.
 *
 * first are start/end except with first < last
 */
#[derive(Default, Debug, Clone, Copy)]
pub struct Selection {
    pub first: Loc,
    pub last: Loc,

    pub start: Loc,
    pub end: Loc,
}

impl Selection {
    pub fn new(start: Loc, end: Loc) -> Self {
        Selection {
            start,
            end,

            first: start.min(end),
            last: start.max(end),
        }
    }

    pub fn empty() -> Self {
        Self::new(Loc::zero(), Loc::zero())
    }

    pub fn with_end(&self, end: Loc) -> Self {
        Selection::new(self.start, end)
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

#[derive(Default, Clone, Debug)]
pub struct TextModel {
    pub lines: im::Vector<String>,
    pub cursor: Loc,
    pub original_cursor: Option<Loc>,
}

impl TextModel {
    pub fn new() -> Self {
        Default::default()
    }

    #[must_use]
    pub fn load_from(&self, path: &PathBuf) -> io::Result<Self> {
        let f = BufReader::new(File::open(path)?);

        Ok(Self {
            lines: f
                .lines()
                .map(|x| x.expect("could not parse line"))
                .collect(),
            ..self.clone()
        })
    }

    #[must_use]
    pub fn move_cursor<D: Into<LocDelta>>(&self, offset: D) -> Self {
        let offset = offset.into();

        Self {
            cursor: if offset.column == 0 {
                self.original_cursor.unwrap_or(self.cursor).offset(offset).constrain(&self.lines)
            } else {
                self.cursor.offset(offset).constrain(&self.lines)
            },
            original_cursor: if offset.column == 0 {
                Some(self.original_cursor.unwrap_or(self.cursor).offset(offset).constrain_only_row(&self.lines))
            } else {
                None
            },
            ..self.clone()
        }
    }

    #[must_use]
    pub fn insert(&self, data: &str) -> Self {
        let row = self.cursor.row;

        // Lines to be inserted inbetween.
        let mut inbetween: im::Vector<String> = data.lines().map(|x| x.to_owned()).collect();
        let cursor = self.cursor.add_wrapped(get_last_cursor(&inbetween));

        if inbetween.is_empty() {
            inbetween.push_back(String::new());
        }

        match self.lines.get(row) {
            Some(line) => {
                let (a, b) = line.split_at(self.cursor.column as usize);
                inbetween.front_mut().unwrap().insert_str(0, a);
                inbetween.back_mut().unwrap().push_str(b);
            }
            _ => (),
        }

        // Copy of the original.
        let mut lines = self.lines.clone();
        let pre = lines.slice(..row);
        let post = lines.slice(1..);

        Self {
            lines: pre + inbetween + post,
            cursor,
            original_cursor: None,
            ..self.clone()
        }
    }

    pub fn backspace_key(&self) -> Self {
        // Cursor at start.
        if self.cursor == Loc::zero() {
            return self.clone();
        }

        // Delete char/row
        let mut lines = self.lines.clone();

        let cursor = if self.cursor.column == 0 {
            let removed = lines.remove(self.cursor.row);
            let mut new_col = 0;

            match lines.get_mut(self.cursor.row - 1) {
                Some(line) => {
                    new_col = line.len();
                    line.push_str(removed.as_str());
                }
                _ => (),
            }

            self.cursor.offset((-1, 0)).with_column(new_col)
        } else {
            match lines.get_mut(self.cursor.row) {
                Some(line) => {
                    line.remove(self.cursor.column - 1);
                }
                _ => (),
            }

            self.cursor.offset((0, -1))
        };

        Self {
            lines,
            cursor,
            original_cursor: None,

            ..self.clone()
        }
    }

    // pub fn delete_key(&self) -> Self {
    //     Self {
    //         ..self.clone()
    //     }
    // }

    pub fn insert_newline(&self) -> Self {
        let mut lines = self.lines.clone();
        let (a, b) = self
            .lines
            .get(self.cursor.row)
            .map(|x| {
                let (from, to) = x.split_at(self.cursor.column);
                (from.to_owned(), to.to_owned())
            })
            .unwrap_or((String::new(), String::new()));

        lines.set(self.cursor.row, a);
        lines.insert(self.cursor.row + 1, b);

        Self {
            lines,
            cursor: self.cursor.add_wrapped((1, 0)),
            original_cursor: None,
            ..self.clone()
        }
    }

    pub fn delete(&self, selection: Selection) -> TextModel {
        let mut lines = self.lines.clone();

        let post = lines.slice((selection.last.row + 1)..);
        let pre = lines.slice(..selection.first.row);

        let inbetween = lines;
        let inbetween = im::Vector::<String>::unit(
                inbetween.front().map(|x| x[..selection.first.column].to_string()).unwrap_or_default() +
                inbetween.back().map(|x| &x[selection.last.column..]).unwrap_or("")
            );

        Self {
            lines: pre + inbetween + post,
            cursor: selection.first,
            original_cursor: None,
            ..self.clone()
        }
    }

    pub fn get_string(&self, selection: Selection) -> String {
        let mut result = String::new();

        for row in selection.first.row..=selection.last.row {
            match self.lines.get(row) {
                Some(line) => {
                    let l = if row == selection.first.row { selection.first.column } else { 0 };
                    let r = if row == selection.last.row { selection.last.column } else { line.len() };

                    result.push_str(&line[l..r]);
                    result.push_str("\n");
                },
                _ => (),
            }
        }

        result
    }

    #[must_use]
    pub fn map<F: FnOnce(&mut Self) -> ()>(&self, f: F) -> Self {
        let mut doc = self.clone();
        f(&mut doc);
        doc
    }
}

impl ToString for TextModel {
    fn to_string(&self) -> String {
        self.lines
            .iter()
            .map(|x| x.as_str())
            .collect::<Vec<&str>>()
            .join("\n")
            .to_owned()
    }
}
