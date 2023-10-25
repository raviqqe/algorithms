pub fn solve(xs: &[(f64, f64)]) -> f64 {
    let n = xs.len();
    let mut dp = vec![vec![f64::INFINITY; n + 1]; 1 << n];
    dp[0][0] = 0.0;

    for i in 0..1 << n {
        for j in 0..n {
            if dp[i][j].is_infinite() {
                continue;
            }

            for k in 0..n {
                if 1 << k & i > 0 {
                    continue;
                }

                let ii = i | 1 << k;

                dp[ii][k] = dp[ii][k].min(dp[i][j] + distance(j, k, xs));
            }
        }
    }

    dp.last().unwrap()[0]
}

fn distance(i: usize, j: usize, xs: &[(f64, f64)]) -> f64 {
    ((xs[i].0 - xs[j].0).powi(2) + (xs[i].1 - xs[j].1).powi(2)).sqrt()
}

fn reconstruct(dp: &[Vec<f64>], mut y: f64) -> () {
    let mut js = vec![];
    let mut i = dp[0].len() - 1;

    while i > 0 {
        let j = dp[i].iter().position(|&x| x == y).unwrap();
        js.push(j);
        i = i & !(1 << j);
    }

    return;
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn simple() {
        assert_eq!(solve(&[(0.0, 0.0), (1.0, 0.0)]), 2.0);
        assert_eq!(
            solve(&[(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)]),
            4.0
        );
        assert_eq!(solve(&[(0.0, 0.0), (1.0, 0.0), (2.0, 0.0)]), 4.0);
    }
}
