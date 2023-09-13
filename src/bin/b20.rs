use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }

    println!("{}", distance(s.as_bytes(), t.as_bytes()));
}

fn distance(s: &[u8], t: &[u8]) -> usize {
    let mut dp = vec![vec![usize::MAX; t.len() + 1]; s.len() + 1];

    for i in 0..=s.len() {
        for j in 0..=t.len() {
            dp[i][j] = if i == 0 && j == 0 {
                0
            } else {
                [
                    if i > 0 && j > 0 && s[i - 1] == t[j - 1] {
                        Some(dp[i - 1][j - 1])
                    } else {
                        None
                    },
                    if i > 0 && j > 0 && s[i - 1] != t[j - 1] {
                        Some(dp[i - 1][j - 1] + 1)
                    } else {
                        None
                    },
                    if i > 0 { Some(dp[i - 1][j] + 1) } else { None },
                    if j > 0 { Some(dp[i][j - 1] + 1) } else { None },
                ]
                .into_iter()
                .flatten()
                .min()
                .unwrap()
            }
        }
    }

    dp[s.len()][t.len()]
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn equal() {
        assert_eq!(distance(b"", b""), 0);
        assert_eq!(distance(b"a", b"a"), 0);
        assert_eq!(distance(b"ab", b"ab"), 0);
        assert_eq!(distance(b"abc", b"abc"), 0);
    }

    #[test]
    fn delete() {
        assert_eq!(distance(b"a", b""), 1);
        assert_eq!(distance(b"ab", b""), 2);
        assert_eq!(distance(b"ab", b"a"), 1);
        assert_eq!(distance(b"ab", b"b"), 1);
        assert_eq!(distance(b"ab", b""), 2);
        assert_eq!(distance(b"abc", b"ac"), 1);
    }

    #[test]
    fn insert() {
        assert_eq!(distance(b"", b"a"), 1);
        assert_eq!(distance(b"", b"ab"), 2);
        assert_eq!(distance(b"", b"abc"), 3);
        assert_eq!(distance(b"a", b"ab"), 1);
        assert_eq!(distance(b"b", b"ab"), 1);
        assert_eq!(distance(b"ac", b"abc"), 1);
    }

    #[test]
    fn replace() {
        assert_eq!(distance(b"a", b"b"), 1);
        assert_eq!(distance(b"ab", b"cd"), 2);
        assert_eq!(distance(b"abc", b"def"), 3);
        assert_eq!(distance(b"abc", b"adc"), 1);
    }
}
