use ordered_float::OrderedFloat;
use proconio::input;

fn main() {
    input! {
        n: usize,
        xs: [(f64, f64); n],
    }

    println!("{}", solve(&xs));
}

fn solve(xs: &[(f64, f64)]) -> f64 {
    (0..xs.len()).map(|i| solve_start(i, xs)).min().unwrap().0
}

fn solve_start(start: usize, xs: &[(f64, f64)]) -> OrderedFloat<f64> {
    let n = xs.len();
    let mut dp = vec![vec![OrderedFloat(f64::INFINITY); n + 1]; 1 << n];
    dp[0][start] = OrderedFloat(0.0);

    for i in 0..1 << n {
        for j in 0..n {
            for k in 0..n {
                if 1 << k & i > 0 {
                    continue;
                }

                let ii = i | 1 << k;

                dp[ii][k] = dp[ii][k].min(dp[i][j] + distance(j, k, xs));
            }
        }
    }

    *dp.last()
        .into_iter()
        .flatten()
        .enumerate()
        .find_map(|(i, x)| (i == start).then_some(x))
        .unwrap()
}

fn distance(j: usize, k: usize, xs: &[(f64, f64)]) -> f64 {
    ((xs[j].0 - xs[k].0).powi(2) + (xs[j].1 - xs[k].1).powi(2)).sqrt()
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
