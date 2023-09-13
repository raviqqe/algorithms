use algorithms::utility::print_table;
use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }

    println!("{}", distance(s.as_bytes(), t.as_bytes()));
}

fn distance(s: &[u8], t: &[u8]) -> usize {
    let mut dp = vec![vec![None; t.len() + 1]; s.len() + 1];
    dp[0][0] = Some(0);

    for i in 0..=s.len() {
        for j in 0..=t.len() {
            if i == 0 && j == 0 {
                continue;
            }

            if i > 0 && j > 0 && s[i - 1] == t[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = match (
                    if i > 0 { dp[i - 1][j] } else { None },
                    if j > 0 { dp[i][j - 1] } else { None },
                ) {
                    (Some(d1), Some(d2)) => Some(d1.min(d2) + 1),
                    (Some(d), None) | (None, Some(d)) => Some(d + 1),
                    (None, None) => {
                        dbg!(i, j);
                        None
                    }
                };
            }
        }
    }

    print_table(&dp);

    dp[s.len()][t.len()].unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn simple() {
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
}
