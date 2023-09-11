pub fn is_match(s: String, p: String) -> bool {
    let xs = s.as_bytes();
    let ys = p.as_bytes();
    let mut i = 0;
    let mut j = 0;

    while j < ys.len() {
        if ys[j] == b'*' {
        } else if ys.get(j + 1) == Some(&b'*') {
            while i < xs.len() && (ys[j] == b'.' || xs[i] == ys[j]) {
                i += 1;
            }
        } else {
            if ys[j] != b'.' && xs.get(i) != Some(&ys[j]) {
                return false;
            }

            i += 1;
        }

        j += 1;
    }

    i == xs.len() && j == ys.len()
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
    }

    #[test]
    fn asterisk() {
        assert_eq!(is_match("".into(), "a*".into()), true);
        assert_eq!(is_match("a".into(), "a*".into()), true);
        assert_eq!(is_match("aa".into(), "a*".into()), true);
        assert_eq!(is_match("aaa".into(), "a*".into()), true);
        assert_eq!(is_match("ab".into(), "..".into()), true);
        assert_eq!(is_match("aaa".into(), "a*a".into()), true);
    }
}
