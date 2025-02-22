<<<<<<< HEAD
=======
//The input handleing of the app

>>>>>>> 627c046 (Added comments to each file for easy understanding)
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, KeyEventKind};

pub struct TextArea {
    content: Vec<String>,  // Each line is a separate string
    cursor_row: usize,
    cursor_col: usize,
}

impl TextArea {
   pub fn new() -> Self {
        Self {
            content: vec![String::new()],
            cursor_row: 0,
            cursor_col: 0,
        }
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) -> bool {
        if key.kind != KeyEventKind::Press {
            return false; // Ignore key releases to prevent duplicates
        }

        match key.code {
            KeyCode::Char(c) => {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    return false; // Ignore Ctrl+key combinations
                }
                self.content[self.cursor_row].insert(self.cursor_col, c);
                self.cursor_col += 1;
            }
            KeyCode::Backspace => {
                if self.cursor_col > 0 {
                    self.content[self.cursor_row].remove(self.cursor_col - 1);
                    self.cursor_col -= 1;
                } else if self.cursor_row > 0 {
                    let prev_len = self.content[self.cursor_row - 1].len();
                    let current_line = self.content.remove(self.cursor_row);
                    self.cursor_row -= 1;
                    self.cursor_col = prev_len;
                    self.content[self.cursor_row].push_str(&current_line);
                }
            }
            KeyCode::Enter => {
                let remaining_text = self.content[self.cursor_row].split_off(self.cursor_col);
                self.content.insert(self.cursor_row + 1, remaining_text);
                self.cursor_row += 1;
                self.cursor_col = 0;
            }
            KeyCode::Left => {
                if self.cursor_col > 0 {
                    self.cursor_col -= 1;
                } else if self.cursor_row > 0 {
                    self.cursor_row -= 1;
                    self.cursor_col = self.content[self.cursor_row].len();
                }
            }
            KeyCode::Right => {
                if self.cursor_col < self.content[self.cursor_row].len() {
                    self.cursor_col += 1;
                } else if self.cursor_row + 1 < self.content.len() {
                    self.cursor_row += 1;
                    self.cursor_col = 0;
                }
            }
            KeyCode::Up => {
                if self.cursor_row > 0 {
                    self.cursor_row -= 1;
                    self.cursor_col = self.cursor_col.min(self.content[self.cursor_row].len());
                }
            }
            KeyCode::Down => {
                if self.cursor_row + 1 < self.content.len() {
                    self.cursor_row += 1;
                    self.cursor_col = self.cursor_col.min(self.content[self.cursor_row].len());
                }
            }
            KeyCode::Esc => return true, // Exit on Esc
            _ => {}
        }
        false
    }

    pub fn get_text(&self) -> String {
        self.content.join("\n")
    }
}

