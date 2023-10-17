use ordered_float::OrderedFloat;

pub fn solve(m: usize, xs: &[(f64, f64)]) -> f64 {
    let n = xs.len();
    let mut dp = vec![vec![vec![f64::INFINITY; n]; m]; 1 << n];

    for i in 0..n {
        dp[1 << i][0][i] = 0.0;
    }

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

                    dp[ii][j][l] = dp[ii][j][l].min(dp[i][j][k] + distance(k, l, xs));

                    if j + 1 < m {
                        dp[ii][j + 1][l] = dp[ii][j + 1][l].min(dp[ii][j][k]);
                    }
                }
            }
        }
    }

    *dp.last()
        .unwrap()
        .last()
        .unwrap()
        .iter()
        .min_by_key(|&&x| OrderedFloat(x))
        .unwrap()
}

fn distance(i: usize, j: usize, xs: &[(f64, f64)]) -> f64 {
    ((xs[i].0 - xs[j].0).powi(2) + (xs[i].1 - xs[j].1).powi(2)).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn vehicle() {
        assert_eq!(solve(1, &[(0.0, 0.0), (1.0, 0.0)]), 1.0);
        assert_eq!(
            solve(1, &[(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)]),
            3.0
        );
        assert_eq!(solve(1, &[(0.0, 0.0), (1.0, 0.0), (2.0, 0.0)]), 2.0);
        assert_eq!(solve(1, &[(0.0, 0.0), (2.0, 0.0), (1.0, 0.0)]), 2.0);
        assert_eq!(solve(1, &[(1.0, 0.0), (2.0, 0.0), (0.0, 0.0)]), 2.0);
        assert_eq!(solve(1, &[(2.0, 0.0), (1.0, 0.0), (0.0, 0.0)]), 2.0);
    }

    #[test]
    fn two_vehicles() {
        assert_eq!(solve(2, &[(0.0, 0.0), (1.0, 0.0)]), 0.0);
    }
}
