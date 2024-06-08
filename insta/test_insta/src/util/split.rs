pub fn split_words2(s: &str) -> Vec<&str> {
    s.split_whitespace().collect()
}

#[test]
fn test_split_words() {
    let words = split_words2("hello from the other side");
    insta::assert_yaml_snapshot!(words);
}
