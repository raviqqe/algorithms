pub fn reverse(x: i32) -> i32 {
    while x != 0 {
        x
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
        assert_eq!(convert("".into(), 1), "".to_string());
        assert_eq!(convert("a".into(), 1), "a".to_string());
        assert_eq!(convert("ab".into(), 1), "ab".to_string());
        assert_eq!(convert("foo".into(), 3), "foo".to_string());
        assert_eq!(convert("paypalishiring".into(), 3), "pahnaplsiigyir");
        assert_eq!(convert("paypalishiring".into(), 4), "pinalsigyahrpi");
    }
}
