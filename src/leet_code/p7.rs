pub fn reverse(mut x: i32) -> i32 {
    let mut y = 0;

    while {
        y *= 10;
        x != 0
    } {
        y += x % 10;
        x /= 10;
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    // spell-checker: disable

    #[test]
    fn test() {
        assert_eq!(reverse(0), 0);
        assert_eq!(reverse(1), 1);
        assert_eq!(reverse(2), 2);
        assert_eq!(reverse(-42), -42);
    }
}
