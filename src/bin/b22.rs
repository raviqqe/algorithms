use proconio::input;

fn main() {
    input! {
        n: usize,
        xs: [usize; n - 1],
        ys: [usize; n - 2],
    }

    println!("{}", solve(&xs, &ys));
}

fn solve(xs: &[usize], ys: &[usize]) -> usize {
    let n = xs.len() + 1;
    let mut dp = vec![usize::MAX; n];
    dp[0] = 0;

    for i in 0..n {
        if i < n - 1 {
            dp[i + 1] = dp[i + 1].min(dp[i] + xs[i]);
        } else if i < n - 2 {
            dp[i + 2] = dp[i + 2].min(dp[i] + ys[i]);
        }
    }

    *dp.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!(solve(&[1, 2], &[0]), 3);
    }
}
