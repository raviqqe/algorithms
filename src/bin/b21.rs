use proconio::{input, marker::Bytes};

fn main() {
    input! {
        _: usize,
        s: Bytes,
    }

    println!("{}", solve(&s));
}

fn solve(s: &[u8]) -> usize {
    let mut dp = vec![vec![0; s.len()]; s.len()];

    for i in (0..s.len()).rev() {
        for j in i..s.len() {
            dp[i][j] = if i == j {
                1
            } else {
                dp[i + 1][j - 1] + if s[i] == s[j] { 2 } else { 0 }
            };
        }
    }

    dp.into_iter().flatten().max().unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn simple() {
        assert_eq!(solve(b""), 0);
        assert_eq!(solve(b"a"), 1);
        assert_eq!(solve(b"aa"), 2);
        assert_eq!(solve(b"aaa"), 3);
    }

    #[test]
    fn complex() {
        assert_eq!(solve(b"ab"), 1);
        assert_eq!(solve(b"aba"), 3);
        assert_eq!(solve(b"abcde"), 1);
        assert_eq!(solve(b"abcbe"), 3);
        assert_eq!(solve(b"abcdcba"), 7);
        assert_eq!(solve(b"programming"), 4);
    }
}
