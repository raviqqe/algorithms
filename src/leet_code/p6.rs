pub fn convert(s: String, row_count: i32) -> String {
    let mut ys = Vec::<u8>::with_capacity(s.len());

    for i in 0..row_count {
        ys.extend(
            if i == 0 || i == row_count - 1 {
                ys.get(i)
            } else {
                ys.get(i)
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
        assert_eq!(convert("".into(), 0), "".to_string());
        assert_eq!(convert("foo".into(), 3), "foo".to_string());
        assert_eq!(convert("PAYPALISHIRING", 3), "PINALSIGYAHRPI");
    }
}
