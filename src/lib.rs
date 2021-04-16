pub fn collatz(n: u64) -> Option<u64> {
    match n {
        0 => None,
        1 => Some(0),
        n if n % 2 == 0 => collatz(n/2) .map(|x| x + 1),
        n => collatz(3 * n + 1).map(|x| x + 1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Some(0), collatz(1));
    }

    #[test]
    #[ignore]
    fn test_16() {
        assert_eq!(Some(4), collatz(16));
    }

    #[test]
    #[ignore]
    fn test_12() {
        assert_eq!(Some(9), collatz(12));
    }

    #[test]
    #[ignore]
    fn test_1000000() {
        assert_eq!(Some(152), collatz(1000000));
    }

    #[test]
    #[ignore]
    fn test_0() {
        assert_eq!(None, collatz(0));
    }
}
