/*!
 * The common crate is a collection of common utilities and types used by the other novella crates.
 * It also re-exports a number of external crates that are used across the novella crates.
 */
#![feature(min_specialization)]
pub mod exports;
pub mod components {
    //pub fn get_unique_type_id<T>() -> u64 {}
}

pub mod type_id {
    pub trait TypeIdTy {
        fn get_type_id() -> TypeId;
        fn get_type_id_ref(&self) -> TypeId;
        fn get_name() -> &'static str;
        fn get_name_ref(&self) -> &'static str;
    }
    ///A globally unique identifier for a type
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        bincode::Encode,
        bincode::Decode,
    )]
    pub struct TypeId {
        pub id: u64,
    }
    impl TypeId {
        pub fn new(id: u64) -> Self {
            Self { id }
        }
        pub fn get_id(&self) -> u64 {
            self.id
        }
        pub fn of<T: TypeIdTy + ?Sized>() -> Self {
            T::get_type_id()
        }
    }
    impl<T: 'static + ?Sized> TypeIdTy for T {
        default fn get_type_id() -> TypeId {
            let ti = std::any::TypeId::of::<T>();
            //reintrepret as u64
            unsafe {
                let id: u64 = std::mem::transmute(ti);
                TypeId { id }
            }
        }
        default fn get_type_id_ref(&self) -> TypeId {
            let ti = std::any::TypeId::of::<T>();
            //reintrepret as u64
            unsafe {
                let id: u64 = std::mem::transmute(ti);
                TypeId { id }
            }
        }
        default fn get_name() -> &'static str {
            std::any::type_name::<T>()
        }
        default fn get_name_ref(&self) -> &'static str {
            std::any::type_name::<T>()
        }
    }
}

pub trait StringExt {
    fn to_snake_case(&self) -> String;
}
impl StringExt for str {
    fn to_snake_case(&self) -> String {
        let mut result = String::new();
        for (i, c) in self.chars().enumerate() {
            if i == self.len() - 1 {
                result.push_str(c.to_lowercase().to_string().as_str());

                break;
            }
            if !c.is_uppercase() && self.chars().nth(i + 1).unwrap().is_uppercase() {
                result.push_str(c.to_lowercase().to_string().as_str());
                result.push('_');
            } else {
                result.push_str(c.to_lowercase().to_string().as_str());
            }
        }
        result
    }
}

pub mod text {
    pub trait TextChunk {
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
    pub struct Selection {
        start: Location,
        end: Location,
    }
    impl Selection {
        pub fn new(start: Location, end: Location) -> Selection {
            Selection { start, end }
        }
        pub fn get_selection_string(&self, text: &str) -> String {
            let (start, end) = self.get_abs_index_range(text);
            text[start..end].to_string()
        }
        ///Get the absolute index range of the selection
        /// Returns (start, end) where start is the index of the first character of the selection
        /// and end is the index of the last character of the selection of the given text
        ///
        pub fn get_abs_index_range(&self, text: &str) -> (usize, usize) {
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
}

pub mod uuid {
    pub fn gen_128() -> u128 {
        uuid::Uuid::new_v4().as_u128()
    }
    pub fn gen_64() -> u64 {
        uuid::Uuid::new_v4().as_u128() as u64
    }
}
///A thin wrapper around std::any::type_name::<T>()
pub fn type_name_any<T: ?Sized>() -> &'static str {
    std::any::type_name::<T>()
}

#[cfg(test)]
mod test_super {

    use super::*;

    #[test]
    fn test_to_snake_case() {
        let test = "TestStringVEN";
        assert_eq!(test.to_snake_case(), "test_string_ven");
    }
    #[test]
    fn test_get_type_name() {}
}
