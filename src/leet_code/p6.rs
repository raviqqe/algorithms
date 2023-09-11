pub fn convert(s: String, row_count: i32) -> String {
    let row_count = row_count as usize;
    let xs = s.as_bytes();
    let mut ys = Vec::<u8>::with_capacity(s.len());

    for i in 0..xs.len() / row_count {
        ys.extend(
            if i == 0 || i == row_count - 1 {
                xs.get(i)
            } else {
                xs.get(i)
            }
            .copied(),
        );
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
        assert_eq!(convert("PAYPALISHIRING".into(), 3), "PINALSIGYAHRPI");
    }
}
