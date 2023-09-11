pub fn convert(s: String, row_count: i32) -> String {
    let row_count = row_count as usize;
    let a = if row_count > 1 {
        2 * (row_count - 1)
    } else {
        1
    };
    let xs = s.as_bytes();
    let mut ys = Vec::<u8>::with_capacity(s.len());

    for i in 0..row_count {
        for j in 0..xs.len() / a + 1 {
            let k = i + j * a;

            ys.extend(xs.get(k).copied());

            if row_count > 1 && i % (row_count - 1) != 0 {
                ys.extend(xs.get(k + a - 2 * i).copied());
            }
        }
    }

    String::from_utf8(ys).unwrap()
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
