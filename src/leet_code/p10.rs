pub fn is_match(s: String, p: String) -> bool {
    dbg!("START");
    let xs = s.as_bytes();
    let ys = p.as_bytes();

    let mut zs = vec![vec![false; xs.len() + 1]; ys.len() + 1];

    zs[0][0] = true;

    for i in 0..ys.len() {
        for j in 0..zs[i].len() {
            if !zs[i][j] {
                continue;
            }

            dbg!(i, j, ys[i], xs.get(j));
            match (ys[i], xs.get(j)) {
                (b'.', Some(_)) => zs[i + 1][j + 1] = true,
                (y, Some(&x)) => zs[i + 1][j + 1] = x == y,
                (_, None) => {}
            }
        }
    }

    zs[ys.len()][xs.len()]
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
        // assert_eq!(is_match("".into(), "a*".into()), true);
        // assert_eq!(is_match("a".into(), "a*".into()), true);
        // assert_eq!(is_match("aa".into(), "a*".into()), true);
        // assert_eq!(is_match("aaa".into(), "a*".into()), true);
        // assert_eq!(is_match("ab".into(), "..".into()), true);
    }

    #[test]
    fn non_exhaustive_asterisk() {
        // assert_eq!(is_match("aaa".into(), "a*a".into()), true);
    }
}
