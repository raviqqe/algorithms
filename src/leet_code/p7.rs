pub fn reverse(mut x: i32) -> i32 {
    let mut y = 0;

    while x != 0 {
        y = y * 10 + x % 10;
        x /= 10;
    }

    y
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test() {
        assert_eq!(reverse(0), 0);
        assert_eq!(reverse(1), 1);
        assert_eq!(reverse(2), 2);
        assert_eq!(reverse(42), 24);
        assert_eq!(reverse(-42), -24);
    }
}
