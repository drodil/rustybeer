pub fn contains_case_insensitive(checked: &String, contain: &String) -> bool {
    checked.to_ascii_lowercase().contains(&contain.to_ascii_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_case_insensitive() {
        assert!(contains_case_insensitive(&"DoIHaveTheSubString".to_owned(), &"substring".to_owned()));
    }
}
