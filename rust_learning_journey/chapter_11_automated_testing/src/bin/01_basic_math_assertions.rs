fn add(a: i32, b: i32) -> i32 {
    a + b
}
#[cfg(test)]
mod test {
    use super::add;
    #[test]
    fn test() {
        assert_eq!(add(10, 20), 30);
        assert_ne!(add(20, 10), 20);
    }
}
