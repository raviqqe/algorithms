pub fn convert(s: String, row_count: i32) -> String {
    let row_count = row_count as usize;
    let a = 2 * (row_count - 1);
    let xs = s.as_bytes();
    let mut ys = Vec::<u8>::with_capacity(s.len());

    for i in 0..row_count {
        for j in 0..xs.len() / a.max(1) + 1 {
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

    #[test]
    fn test() {
        assert_eq!(convert("".into(), 1), "".to_string());
        assert_eq!(convert("foo".into(), 3), "foo".to_string());
        assert_eq!(convert("PAYPALISHIRING".into(), 3), "PAHNAPLSIIGYIR");
        assert_eq!(convert("PAYPALISHIRING".into(), 4), "PINALSIGYAHRPI");
    }
}
