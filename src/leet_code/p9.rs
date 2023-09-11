pub fn is_palindrome(x: i32) -> bool {
    match x {
        0 => true,
        x if x < 0 => false,
        x => {
            let n = x.ilog10();
            let m = 10i32.pow(n);
            let mut d = 1;

            for _ in 0..(n + 1) / 2 {
                if digit(x, d) != digit(x, m / d) {
                    return false;
                }

                d *= 10;
            }

            true
        }
    }
}

fn digit(x: i32, y: i32) -> i32 {
    x / y % 10
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
