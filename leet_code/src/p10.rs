pub fn is_match(s: String, p: String) -> bool {
    let xs = s.as_bytes();
    let ys = p.as_bytes();

    let mut zs = vec![vec![false; ys.len() + 1]; xs.len() + 1];

    zs[0][0] = true;

    for i in 0..=xs.len() {
        for j in 0..ys.len() {
            zs[i][j + 1] = ys[j] == b'*'
                && (zs[i][j - 1]
                    || zs[i][j]
                    || i > 0 && (zs[i - 1][j + 1] && is_byte_match(xs[i - 1], ys[j - 1])))
                || i > 0 && zs[i - 1][j] && is_byte_match(xs[i - 1], ys[j]);
        }
    }

    zs[xs.len()][ys.len()]
}

fn is_byte_match(x: u8, y: u8) -> bool {
    [x, b'.'].contains(&y)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    // spell-checker: disable

    #[test]
    fn simple() {
        assert_eq!(is_match("".into(), "".into()), true);
        assert_eq!(is_match("a".into(), "".into()), false);
        assert_eq!(is_match("".into(), "a".into()), false);
        assert_eq!(is_match("a".into(), "a".into()), true);
        assert_eq!(is_match("ab".into(), "a".into()), false);
        assert_eq!(is_match("a".into(), "ab".into()), false);
        assert_eq!(is_match("ab".into(), "ab".into()), true);
    }

    #[test]
    fn dot() {
        assert_eq!(is_match("a".into(), ".".into()), true);
        assert_eq!(is_match("ab".into(), "..".into()), true);
        assert_eq!(is_match("".into(), ".".into()), false);
    }

    #[test]
    fn asterisk() {
        assert_eq!(is_match("abbc".into(), "ab*c".into()), true);
        assert_eq!(is_match("".into(), "a*".into()), true);
        assert_eq!(is_match("a".into(), "a*".into()), true);
        assert_eq!(is_match("aa".into(), "a*".into()), true);
        assert_eq!(is_match("aaa".into(), "a*".into()), true);
        assert_eq!(is_match("ab".into(), "..".into()), true);
        assert_eq!(is_match("".into(), "aa*".into()), false);
    }

    #[test]
    fn non_exhaustive_asterisk() {
        assert_eq!(is_match("aaa".into(), "a*a".into()), true);
    }

    #[test]
    fn multiple_asterisks() {
        assert_eq!(is_match("mississippi".into(), "mis*is*p*.".into()), false);
    }
}
