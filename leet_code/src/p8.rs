pub fn parse(s: String) -> i32 {
    let xs = s.as_bytes();
    let mut i = 0;
    let mut y = 0i32;

    while xs.get(i) == Some(&b' ') {
        i += 1;
    }

    let s = match xs.get(i) {
        Some(b'-') => {
            i += 1;
            -1
        }
        Some(b'+') => {
            i += 1;
            1
        }
        _ => 1,
    };

    while is_digit(xs.get(i)) {
        let x = xs[i];

        y = y.saturating_mul(10).saturating_add(s * (x - b'0') as i32);
        i += 1;
    }

    y
}

const fn is_digit(x: Option<&u8>) -> bool {
    matches!(x, Some(b'0'..=b'9'))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn simple() {
        assert_eq!(parse("0".into()), 0);
        assert_eq!(parse("1".into()), 1);
        assert_eq!(parse("+1".into()), 1);
        assert_eq!(parse("-1".into()), -1);
    }

    #[test]
    fn digits() {
        assert_eq!(parse("42".into()), 42);
        assert_eq!(parse("1234".into()), 1234);
        assert_eq!(parse("+42".into()), 42);
        assert_eq!(parse("+1234".into()), 1234);
        assert_eq!(parse("-42".into()), -42);
        assert_eq!(parse("-1234".into()), -1234);
    }

    #[test]
    fn space() {
        assert_eq!(parse(" 1".into()), 1);
        assert_eq!(parse("  1".into()), 1);
    }

    #[test]
    fn invalid() {
        assert_eq!(parse("words and 987".into()), 0);
        assert_eq!(parse("+-12".into()), 0);
    }

    #[test]
    fn big() {
        assert_eq!(parse("-91283472332".into()), -2147483648);
    }
}
