pub const fn is_palindrome(x: i32) -> bool {
    x >= 0
        && x == {
            let mut x = x;
            let mut y = 0;

            while x != 0 {
                y = y * 10 + x % 10;
                x /= 10;
            }

            y
        }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn simple() {
        assert_eq!(is_palindrome(0), true);
        assert_eq!(is_palindrome(-1), false);
        assert_eq!(is_palindrome(1), true);
        assert_eq!(is_palindrome(10), false);
        assert_eq!(is_palindrome(11), true);
        assert_eq!(is_palindrome(121), true);
        assert_eq!(is_palindrome(123), false);
        assert_eq!(is_palindrome(12321), true);
        assert_eq!(is_palindrome(1232), false);
    }
}
