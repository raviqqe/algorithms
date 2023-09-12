pub fn is_match(s: String, p: String) -> bool {
    dbg!("START");
    let xs = s.as_bytes();
    let ys = p.as_bytes();

    let mut zs = vec![vec![false; ys.len() + 1]; xs.len() + 1];

    for i in 0..xs.len() + 1 {
        for j in 0..ys.len() + 1 {
            let ii = i.wrapping_sub(1);
            let jj = j.wrapping_sub(1);
            let z = zs.get(ii).and_then(|zs| zs.get(jj));

            zs[i][j] = match (xs.get(ii), ys.get(jj), z) {
                (None, None, _) => true,
                (None, Some(b'*'), _) => zs[i][j - 2],
                (Some(&x), Some(b'*'), _) => zs[i][j - 2] || x == ys[j - 2],
                (None, Some(_), _) => false,
                (Some(_), None, _) => false,
                (Some(_), Some(b'.'), Some(&z)) => z && true,
                (Some(x), Some(y), Some(&z)) => z && x == y,
                (Some(_), Some(_), None) => unreachable!(),
            };
        }
    }
    dbg!(&zs);

    zs[xs.len()][ys.len()]
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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
}
