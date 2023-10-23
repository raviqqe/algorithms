use proconio::input;

fn main() {
    input! {
      n: usize,
      m: usize,
      xs: [usize; n],
    }

    println!("{}", solve(&xs, m));
}

fn solve(xs: &[usize], m: usize) -> bool {
    let mut dp = vec![vec![false; m + 1]; xs.len() + 1];
    dp[0][0] = true;

    for i in 1..=xs.len() {
        for j in 0..=m {
            let x = xs[i - 1];

            dp[i][j] = (0..=m / x).any(|k| {
                let w = k * x;
                j >= w && dp[i - 1][j - w]
            });
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
        assert_eq!(solve(&[1], 2), true);

        assert_eq!(solve(&[1, 2], 0), true);
        assert_eq!(solve(&[1, 2], 1), true);
        assert_eq!(solve(&[1, 2], 2), true);
        assert_eq!(solve(&[1, 2], 3), true);
        assert_eq!(solve(&[1, 2], 4), true);

        assert_eq!(solve(&[2, 2], 0), true);
        assert_eq!(solve(&[2, 2], 1), false);
        assert_eq!(solve(&[2, 2], 2), true);
        assert_eq!(solve(&[2, 2], 3), false);
        assert_eq!(solve(&[2, 2], 4), true);
    }

    #[test]
    fn duplicate() {
        assert_eq!(solve(&[1], 0), true);
        assert_eq!(solve(&[1], 1), true);
        assert_eq!(solve(&[1], 2), true);
        assert_eq!(solve(&[1], 3), true);

        assert_eq!(solve(&[2, 3], 0), true);
        assert_eq!(solve(&[2, 3], 1), false);
        assert_eq!(solve(&[2, 3], 2), true);
        assert_eq!(solve(&[2, 3], 3), true);
        assert_eq!(solve(&[2, 3], 4), true);
        assert_eq!(solve(&[2, 3], 5), true);
        assert_eq!(solve(&[2, 3], 6), true);
        assert_eq!(solve(&[2, 3], 7), true);

        assert_eq!(solve(&[3, 5], 0), true);
        assert_eq!(solve(&[3, 5], 1), false);
        assert_eq!(solve(&[3, 5], 2), false);
        assert_eq!(solve(&[3, 5], 3), true);
        assert_eq!(solve(&[3, 5], 4), false);
        assert_eq!(solve(&[3, 5], 5), true);
        assert_eq!(solve(&[3, 5], 6), true);
        assert_eq!(solve(&[3, 5], 7), false);
    }
}
