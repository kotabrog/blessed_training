mod util;

fn split_words(s: &str) -> Vec<&str> {
    s.split_whitespace().collect()
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_split_words() {
    let words = split_words("hello from the other side");
    insta::assert_yaml_snapshot!(words);
}

#[test]
fn test_sample2() {
    insta::assert_yaml_snapshot!(42);
}

#[test]
fn test_inline() {
    insta::assert_yaml_snapshot!(vec![1, 2, 3], @r###"
    ---
    - 1
    - 2
    - 3
    "###);
}

#[test]
fn test_debug_expressions() {
    insta::assert_yaml_snapshot!("snapshot", "value", "description");
}

#[test]
fn test_redaction() {
    let vec = vec![1, 2, 3];
    insta::assert_yaml_snapshot!(vec, {
        "[2]" => 10,
    });
}
