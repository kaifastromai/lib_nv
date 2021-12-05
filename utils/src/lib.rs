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

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_to_snake_case() {
        let test = "TestStringVEN";
        assert_eq!(test.to_snake_case(), "test_string_ven");
    }
}
