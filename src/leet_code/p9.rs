pub fn is_palindrome(x: i32) -> bool {
    x >= 0 && {
        let n = x.ilog10();

        (0..n / 2).all(|i| digit(x, i) == digit(x, n - i))
    }
}

fn digit(x: i32, y: u32) -> i32 {
    x / 10i32.pow(y) % 10
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn simple() {
        assert_eq!(is_palindrome(-1), false);
        assert_eq!(is_palindrome(1), true);
        assert_eq!(is_palindrome(11), true);
        assert_eq!(is_palindrome(121), true);
        assert_eq!(is_palindrome(123), false);
        assert_eq!(is_palindrome(12321), true);
        assert_eq!(is_palindrome(1232), false);
    }
}
