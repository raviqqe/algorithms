pub fn atoi(s: String) -> i32 {
    let xs = s.as_bytes();
    let mut i = 0;
    let mut y = 0i32;

    while !is_digit(xs[i]) && ![b'+', b'-'].contains(&xs[i]) {
        i += 1;
    }

    let s = if xs[i] == b'-' { -1 } else { 1 };
    dbg!(s);

    while !is_digit(xs[i]) {
        i += 1;
    }

    while i < xs.len() && is_digit(xs[i]) {
        let x = xs[i];

        y = y * 10 + s * (x - b'0') as i32;
        i += 1;
    }

    y
}

fn is_digit(x: u8) -> bool {
    matches!(x, b'0'..=b'9')
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn simple() {
        assert_eq!(atoi("0".into()), 0);
        assert_eq!(atoi("1".into()), 1);
        assert_eq!(atoi("+1".into()), 1);
        assert_eq!(atoi("-1".into()), -1);
    }

    #[test]
    fn digits() {
        assert_eq!(atoi("42".into()), 42);
        assert_eq!(atoi("1234".into()), 1234);
        assert_eq!(atoi("+42".into()), 42);
        assert_eq!(atoi("+1234".into()), 1234);
        assert_eq!(atoi("-42".into()), -42);
        assert_eq!(atoi("-1234".into()), -1234);
    }

    #[test]
    fn space() {
        assert_eq!(atoi(" 1".into()), 1);
        assert_eq!(atoi("  1".into()), 1);
    }
}
