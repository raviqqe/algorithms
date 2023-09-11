pub fn reverse(mut x: i32) -> i32 {
    let mut y = 0i32;

    while x != 0 {
        let Some(z) = y.checked_mul(10).and_then(|y| y.checked_add(x % 10)) else {
            return 0;
        };

        y = z;
        x /= 10;
    }

    y
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn simple() {
        assert_eq!(reverse(0), 0);
        assert_eq!(reverse(1), 1);
        assert_eq!(reverse(2), 2);
        assert_eq!(reverse(42), 24);
        assert_eq!(reverse(-42), -24);
        assert_eq!(reverse(120), 21);
        assert_eq!(reverse(1534236469), 0);
    }
}
