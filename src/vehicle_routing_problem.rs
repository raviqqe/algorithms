pub fn solve(m: usize, xs: &[(f64, f64)]) -> f64 {
    let n = xs.len();
    let mut dp = vec![vec![vec![f64::INFINITY; n + 1]; m]; 1 << n];
    dp[0][0][0] = 0.0;

    for i in 0..1 << n {
        for j in 0..m {
            for k in 0..n {
                if dp[i][j][k].is_infinite() {
                    continue;
                }

                for l in 0..n {
                    if 1 << l & i > 0 {
                        continue;
                    }

                    let ii = i | 1 << l;

                    dp[ii][j][k] = dp[ii][j][l].min(dp[i][j][k] + distance(k, l, xs));

                    if j + 1 < m {
                        dp[ii][j + 1][k] = dp[ii][l].min(dp[i][j] + distance(j, l, xs));
                    }
                }
            }
        }
    }

    // dp.last().unwrap()[0]
    todo!()
}

fn distance(i: usize, j: usize, xs: &[(f64, f64)]) -> f64 {
    ((xs[i].0 - xs[j].0).powi(2) + (xs[i].1 - xs[j].1).powi(2)).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn simple() {
        assert_eq!(solve(1, &[(0.0, 0.0), (1.0, 0.0)]), 2.0);
        assert_eq!(
            solve(1, &[(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)]),
            4.0
        );
        assert_eq!(solve(1, &[(0.0, 0.0), (1.0, 0.0), (2.0, 0.0)]), 4.0);
    }
}
