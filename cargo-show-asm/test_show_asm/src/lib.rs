use std::collections::HashMap;

#[inline(never)]
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

pub fn create_map() -> HashMap<i32, i32> {
    let mut map = HashMap::new();
    map.insert(1, 2);
    map.insert(3, 4);
    map
}

fn not_public() {
    println!("This function is not public");
}

pub fn generic<T>(x: T) -> T {
    x
}

#[inline(never)]
pub fn generic_wrapper(x: i32) -> i32 {
    generic(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}
