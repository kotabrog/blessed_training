fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test2() {
        assert_eq!(1 + 1, 2);
    }

    #[test]
    fn test3() {
        loop {}
    }

    /// https://nexte.st/book/leaky-tests#leaky-tests-nextest-detects
    #[test]
    fn test_subprocess_doesnt_exit() {
        let mut cmd = std::process::Command::new("sleep");
        cmd.arg("120");
        cmd.spawn().unwrap();
    }
}
