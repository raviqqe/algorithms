pub fn longest_palindrome(s: String) -> String {
    let xs = s.as_bytes();
    let mut s = b"" as &[u8];

    for i in 0..xs.len() {
        for j in i..xs.len() {
            let ys = &xs[i..j + 1];

            for k in 0..2 {
                let j = j + k;

                if j + ys.len() > xs.len() {
                    continue;
                }

                let zs = &xs[j..j + ys.len()];

                if ys.len() == zs.len() && ys.iter().zip(zs.iter().rev()).all(|(y, z)| y == z) {
                    let vs = &xs[i..j + ys.len()];

                    if vs.len() > s.len() {
                        s = vs;
                    }
                }
            }
        }
    }

    String::from_utf8(s.to_vec()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn zero() {
        assert_eq!(longest_palindrome("".into()), "".to_string());
    }

    #[test]
    fn one() {
        assert_eq!(longest_palindrome("a".into()), "a".to_string());
        assert_eq!(longest_palindrome("ab".into()), "a".to_string());
    }

    #[test]
    fn two() {
        assert_eq!(longest_palindrome("aa".into()), "aa".to_string());
        assert_eq!(longest_palindrome("aab".into()), "aa".to_string());
        assert_eq!(longest_palindrome("baa".into()), "aa".to_string());
    }

    #[test]
    fn more() {
        assert_eq!(longest_palindrome("abcba".into()), "abcba".to_string());
        assert_eq!(longest_palindrome("xabcbay".into()), "abcba".to_string());
    }
}
