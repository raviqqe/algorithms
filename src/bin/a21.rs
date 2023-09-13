use algorithms::utility::print_table;
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
                        let (y, v) = xs[i - 1];

                        Some(dp[i - 1][j] + if i - 1 < y && y < xs.len() - j { v } else { 0 })
                    } else {
                        None
                    },
                    if j > 0 {
                        let (y, v) = xs[xs.len() - j];

                        Some(dp[i][j - 1] + if i < y + 1 && y < xs.len() - j { v } else { 0 })
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

    println!("TABLE");
    print_table(&dp);

    dp.into_iter().flatten().max().unwrap_or_default()
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
    fn simple() {
        assert_eq!(solve(&[(0, 1)]), 0);
        assert_eq!(solve(&[(1, 1), (1, 1)]), 1);
    }
}
