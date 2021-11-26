use super::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
const TEST_STRING: &str = r#"use super::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
#[test]
fn test_range_deletion() {
    let mut text = EditableText::new();
    //open file test.rs
    let path = Path::new("src/nv/text_edit/test.txt");
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    //convert file to string
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => println!("successfully read {}", display),
    }
    //set text to string
    text.set_str(&s);
    text.set_selection(Selection::new(
        Location::new(0, 0),
        text.text.get_last_location(),
    ));
    //delete range
    text.delete_selection();
    println!("{}", text.text);
    //assert that the text is now empty
    assert_eq!(text.text, "");"#;
const TEST_STRING_SHORT: &str = r#"use super::*;
use std::fs::File;
"#;

#[test]
fn test_get_lines() {
    let mut text = TEST_STRING_SHORT;
    let lines = text.get_lines();
    //convert lines back to string using iterator and assert that it is the same as the original
    let s = lines.iter().map(|x| x.to_string()).collect::<String>();
    assert_eq!(s, text);
}
#[test]
fn test_range_deletion() {
    let mut text = EditableText::new();

    //open file test.rs
    let path = Path::new("src/nv/text_edit/test.txt");
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    //convert file to string
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => println!("successfully read {}", display),
    }
    //set text to string
    text.set_str(&s);
    text.set_selection(Selection::new(
        Location::new(0, 0),
        text.text.get_last_location(),
    ));
    //delete range
    text.delete_selection();
    println!("{}", text.text);
    //assert that the text is now empty
    assert_eq!(text.text, "");
}
#[test]
fn test_abs_to_location() {
    let abs_index = TEST_STRING_SHORT.len() - 1;
    let location = TEST_STRING_SHORT
        .to_string()
        .get_location_from_abs_index(abs_index);
    let returned_index = TEST_STRING_SHORT.to_string().get_abs_index(location);

    assert_eq!(location, TEST_STRING_SHORT.to_string().get_last_location());
}

#[test]
fn test_location_to_abs() {}
#[test]
fn test_range_selection() {
    let mut text = EditableText::new_from_str(TEST_STRING);
    //get size of first line of text
    let first_line_size = text.text.lines().next().unwrap().len();

    //set text to string
    text.set_selection(Selection::new(
        Location::new(0, 0),
        Location::new(0, first_line_size),
    ));
    let selected_text = text
        .get_selection()
        .unwrap()
        .get_selection_string(&text.text);

    //assert text is equal to first line of file
    assert_eq!(selected_text, text.get_line(0));
}
#[test]
fn test_get_word_at_cursor(){
    let mut text = EditableText::new_from_str(TEST_STRING);
    //get size of first line of text
    //set the cursor to start of first line
    text.set_cursor_pos(Location::new(0, 0));
    let test_word=text.get_word_at_cursor();
    //manually get first word of first line
    let first_word=text.get_line(0).split_whitespace().next().unwrap();
    assert_eq!(test_word,first_word);
}
