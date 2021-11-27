#[allow(unused_imports)]
trait TextChunk {
    fn get_last_location(&self) -> Location;
    fn get_range_selection(&self, range: Selection) -> String;
    fn get_abs_index(&self, location: Location) -> usize;
    fn get_location_from_abs_index(&self, abs_index: usize) -> Location;
    fn get_lines(&self) -> Vec<String>;
}

impl TextChunk for str {
    fn get_last_location(&self) -> Location {
        let lines = self.get_lines();

        let loc = Location {
            line: lines.len() - 1,
            column: lines.last().unwrap().len() - 1,
        };
        loc
    }
    fn get_range_selection(&self, range: Selection) -> String {
        range.get_selection_string(self)
    }
    fn get_abs_index(&self, location: Location) -> usize {
        let lines = self.get_lines();
        let mut index = 0;
        (0..=location.line).for_each(|i| {
            if i == location.line {
                index += location.column;
            } else {
                index += lines[i].len();
            }
        });
        index
    }
    fn get_location_from_abs_index(&self, abs_index: usize) -> Location {
        let lines = self.get_lines();
        let mut index = 0;
        let mut line = 0;
        (0..lines.len()).for_each(|i| {
            if index + lines[i].len() > abs_index {
                line = i;
                return;
            }
            index += lines[i].len();
        });
        Location {
            line,
            column: abs_index - index,
        }
    }
    //Split the string into lines, including newlines and hanging empty newlines
    fn get_lines(&self) -> Vec<String> {
        //loop over every unicode character, break at new line
        let mut lines = vec![String::new()];
        self.chars().enumerate().for_each(|(i, c)| {
            lines.last_mut().unwrap().push(c);
            if c == '\n' && i != self.len() - 1 {
                lines.push(String::new());
            }
        });

        lines
    }
}

struct Selection {
    start: Location,
    end: Location,
}
impl Selection {
    fn new(start: Location, end: Location) -> Selection {
        Selection { start, end }
    }
    fn get_selection_string(&self, text: &str) -> String {
        let (start, end) = self.get_abs_index_range(text);
        text[start..end].to_string()
    }
    ///Get the absolute index range of the selection
    /// Returns (start, end) where start is the index of the first character of the selection
    /// and end is the index of the last character of the selection of the given text
    ///
    fn get_abs_index_range(&self, text: &str) -> (usize, usize) {
        let lines = text.lines().collect::<Vec<&str>>();
        let mut start = self.start;
        let mut end = self.end;
        if start > end {
            std::mem::swap(&mut start, &mut end);
        }
        let mut start_index = 0;
        let mut end_index = 0;
        if start.line == end.line {
            start_index = start.column;
            end_index = end.column;
        } else {
            //find start index
            start_index = text.to_string().get_abs_index(start);
            end_index += start_index;

            //find end index
            (start.line..=end.line).for_each(|i| {
                let line = lines[i];
                if i == start.line {
                    end_index += line.len() + 1 - start.column;
                }
                //must acccount for the newline
                else if i == end.line {
                    end_index += end.column + 1;
                } else {
                    end_index += line.len() + 1;
                }
            });
        }

        (start_index, end_index)
    }
}

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

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}
impl Location {
    pub fn new(line: usize, column: usize) -> Self {
        Location { line, column }
    }
}
//override range for Location

//impl addassign for Location
impl std::ops::AddAssign<usize> for Location {
    fn add_assign(&mut self, rhs: usize) {
        self.column += rhs;
    }
}

#[cfg(test)]
mod test;
