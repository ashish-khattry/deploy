/// Adds to usize numbers, and then return final sum of that numbers
///
/// #Arguments
///
/// left-The first number (usize)
/// right-The second number (usize)
///
/// ```
/// let result=my_module::add(10,20);
/// assert_eq!(result,30);
///
/// ```
///
pub fn add(left: usize, right: usize) -> usize {
    left + right
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_add() {
        assert_eq!(add(10, 20), 30);
    }
}
