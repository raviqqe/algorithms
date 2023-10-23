use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        xs: [usize; n],
    }

    println!("{}", if solve(&xs, m) { "Yes" } else { "No" });
}

fn solve(xs: &[usize], m: usize) -> bool {
    let mut dp = vec![vec![false; m + 1]; xs.len() + 1];
    dp[0][0] = true;

    for i in 1..=xs.len() {
        for j in 0..=m {
            dp[i][j] = dp[i - 1][j] || j >= xs[i - 1] && dp[i - 1][j - xs[i - 1]];
        }
    }

    dp[xs.len()][m]
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn simple() {
        assert_eq!(solve(&[], 0), true);
        assert_eq!(solve(&[], 1), false);

        assert_eq!(solve(&[1], 0), true);
        assert_eq!(solve(&[1], 1), true);
        assert_eq!(solve(&[1], 2), false);

        assert_eq!(solve(&[1, 2], 0), true);
        assert_eq!(solve(&[1, 2], 1), true);
        assert_eq!(solve(&[1, 2], 2), true);
        assert_eq!(solve(&[1, 2], 3), true);
        assert_eq!(solve(&[1, 2], 4), false);

        assert_eq!(solve(&[2, 2], 0), true);
        assert_eq!(solve(&[2, 2], 1), false);
        assert_eq!(solve(&[2, 2], 2), true);
        assert_eq!(solve(&[2, 2], 3), false);
        assert_eq!(solve(&[2, 2], 4), true);
    }
}
