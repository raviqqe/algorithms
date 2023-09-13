use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        xs: [(Usize1, usize); n],
    }

    println!("{}", solve(&xs));
}

fn solve(xs: &[(usize, usize)]) -> usize {
    // left * right
    let mut dp = vec![vec![0; xs.len() + 1]; xs.len() + 1];

    for i in 0..=xs.len() {
        for j in 0..=xs.len() {
            dp[i][j] = if i == 0 && j == 0 {
                0
            } else {
                [
                    if i > 0 {
                        let (k, v) = xs[i - 1];

                        Some(dp[i - 1][j] + take(i, j, k, v, xs.len()))
                    } else {
                        None
                    },
                    if j > 0 {
                        let (k, v) = xs[xs.len() - j];

                        Some(dp[i][j - 1] + take(i, j, k, v, xs.len()))
                    } else {
                        None
                    },
                ]
                .into_iter()
                .flatten()
                .max()
                .unwrap()
            }
        }
    }

    dp.into_iter().flatten().max().unwrap_or_default()
}

fn take(i: usize, j: usize, k: usize, v: usize, n: usize) -> usize {
    if i < k + 1 && k < n - j {
        v
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn nothing() {
        assert_eq!(solve(&[]), 0);
    }

    #[test]
    fn take_left() {
        assert_eq!(solve(&[(1, 1)]), 0);
        assert_eq!(solve(&[(1, 1), (2, 1)]), 1);
        assert_eq!(solve(&[(1, 1), (2, 1), (3, 1)]), 2);
    }

    #[test]
    fn take_right() {
        assert_eq!(solve(&[(0, 1)]), 0);
        assert_eq!(solve(&[(1, 1), (0, 1)]), 1);
        assert_eq!(solve(&[(0, 1), (0, 1), (1, 1)]), 2);
        assert_eq!(solve(&[(0, 1), (0, 1), (1, 1), (2, 1)]), 3);
    }

    #[test]
    fn complex() {
        assert_eq!(solve(&[(3, 20), (2, 30), (1, 40), (0, 10)]), 60);
    }
}
