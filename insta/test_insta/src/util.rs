mod split;

#[cfg(test)]
mod tests {
    use super::split::split_words2;

    #[test]
    fn test_split_words() {
        let words = split_words2("hello from the other side");
        insta::assert_yaml_snapshot!(words);
    }
}
