#[expect(clippy::missing_panics_doc)]
pub fn longest_palindrome(s: String) -> String {
    let xs = s.as_bytes();
    let mut ys = b"" as &[u8];

    for i in 0..xs.len() {
        for k in 0..2 {
            for j in 0.. {
                if i >= j && xs.get(i - j) == xs.get(i + j + k) {
                    let zs = &xs[i - j..i + j + k + 1];

                    if zs.len() > ys.len() {
                        ys = zs;
                    }
                } else {
                    break;
                }
            }
        }
    }

    String::from_utf8(ys.to_vec()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    // spell-checker: disable

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
        assert_eq!(longest_palindrome("aaaa".into()), "aaaa".to_string());
        assert_eq!(longest_palindrome("abcba".into()), "abcba".to_string());
        assert_eq!(longest_palindrome("xabcbay".into()), "abcba".to_string());
    }
}
