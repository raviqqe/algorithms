use proconio::{input, marker::Bytes};

fn main() {
    input! {
        _: usize,
        s: Bytes,
    }

    println!("{}", solve(&s));
}

fn solve(s: &[u8]) -> usize {
    let mut dp = vec![vec![false; s.len()]; s.len()];

    for i in 0..s.len() {
        for j in i..s.len() {
            dp[i][j] = i == j || false;
        }
    }

    dp.into_iter()
        .enumerate()
        .flat_map(|(i, dp)| {
            dp.into_iter()
                .enumerate()
                .flat_map(move |(j, b)| b.then_some((i as isize - j as isize).unsigned_abs()))
        })
        .max()
        .map(|x| x + 1)
        .unwrap_or_default()
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
}
