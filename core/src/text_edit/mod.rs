use utils::text::*;

#[allow(unused_imports)]

pub struct EditableText {
    text: String,
    cursor_pos: Location,
    selection: Option<Selection>,
}
impl EditableText {
    fn new() -> Self {
        EditableText {
            text: String::new(),
            cursor_pos: Location { line: 0, column: 0 },
            selection: None,
        }
    }
    fn new_from_str(text: &str) -> Self {
        EditableText {
            text: text.to_string(),
            cursor_pos: Location { line: 0, column: 0 },
            selection: None,
        }
    }
    fn set_str(&mut self, text: &str) {
        self.text = text.to_string();
    }
    fn insert(&mut self, c: char) {
        self.text.insert(self.cursor_pos.column, c);
        self.cursor_pos.column += 1;
    }
    fn delete(&mut self) {
        if self.cursor_pos.column > 0 {
            self.text.remove(self.cursor_pos.column - 1);
            self.cursor_pos.column -= 1;
        }
    }
    fn set_cursor_pos(&mut self, pos: Location) {
        if pos < self.text.get_last_location() {
            self.cursor_pos = pos;
        }
    }
    fn increment_cursor_pos(&mut self) {
        if self.cursor_pos.column
            < self
                .text
                .split('\n')
                .nth(self.cursor_pos.line)
                .unwrap()
                .len()
        {
            self.cursor_pos.column += 1;
        } else {
            self.cursor_pos.line += 1;
            self.cursor_pos.column = 0;
        }
    }
    fn get_cursor_pos(&self) -> Location {
        self.cursor_pos
    }
    fn get_text(&self) -> &str {
        &self.text
    }
    fn get_selection(&self) -> Option<&Selection> {
        self.selection.as_ref()
    }
    fn set_selection(&mut self, selection: Selection) {
        self.selection = Some(selection);
    }
    fn delete_selection(&mut self) -> String {
        let (start, end) = self
            .selection
            .as_ref()
            .expect("No selection has been made")
            .get_abs_index_range(&self.text);
        self.text.drain(start..end).collect()
    }
    fn get_line(&self, line: usize) -> &str {
        let mut line_text = self.text.split('\n');
        line_text.nth(line).expect("Index out of bounds!")
    }
    fn get_char_at(&self, pos: Location) -> char {
        self.text.chars().nth(pos.column).unwrap_or('\0')
    }
    fn get_char_at_cursor(&self) -> char {
        self.get_char_at(self.cursor_pos)
    }
    fn get_word_at_cursor(&self) -> String {
        const WORD_BOUNDARY: [char; 4] = ['\n', ' ', '\t', '\r'];
        let pos = self.cursor_pos;

        let abs_index = self.text.get_abs_index(pos);

        let mut i = abs_index;
        //if we are at a boundary
        if WORD_BOUNDARY.contains(&self.text.chars().nth(i).unwrap()) {
            //iterate forward for nearest character
            let mut forward_path_len: usize = 0;
            while WORD_BOUNDARY.contains(&self.text.chars().nth(i).unwrap()) {
                forward_path_len += 1;
            }

            //iterate backward for nearest character
            let mut backward_path_len: usize = 0;
            while WORD_BOUNDARY.contains(&self.text.chars().nth(i).unwrap()) {
                backward_path_len += 1;
            }
            //compare abs index of forward and backward paths
            if forward_path_len < backward_path_len {
                i += forward_path_len;
            } else {
                i -= backward_path_len;
            }
        }

        //iterate back to end of word
        while i > 0 && !WORD_BOUNDARY.contains(&self.text.chars().nth(i).unwrap()) {
            i -= 1;
        }
        let word_start = i;
        //iterate forward to start of word
        i = abs_index;
        while i < self.text.len() && !WORD_BOUNDARY.contains(&self.text.chars().nth(i).unwrap()) {
            i += 1;
        }
        let word_end = i;
        self.text.get_lines()[pos.line][word_start..word_end].to_string()
    }
    fn move_cursor_to_next_word() {}
}
///Represents a view into a subselection of an EditText, that will automatically update as the range changes
struct TextView<'a> {
    text: &'a str,

    text_range: (usize, usize),
    current_index: usize,
}
impl<'a> TextView<'a> {
    fn new(text: &'a str, text_range: (usize, usize)) -> Self {
        TextView {
            text,
            text_range,
            current_index: 0,
        }
    }
    fn get_range_string(&self) -> String {
        String::from(&self.text[self.text_range.0..self.text_range.1])
    }
}

#[cfg(test)]
mod test;
