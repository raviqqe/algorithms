use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        xs: [(usize, usize); n],
    }

    println!("{:?}", solve(&xs, m));
}

fn solve(xs: &[(usize, usize)], m: usize) -> usize {
    let mut dp = vec![vec![0; m + 1]; xs.len() + 1];

    for i in 1..=xs.len() {
        for j in 0..m {
            let (w, v) = xs[i - 1];
            dp[i][j] = dp[i - 1][j].max(if j >= w { dp[i - 1][j - w] + v } else { 0 });
        }
    }

    dp[xs.len()].iter().copied().max().unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!(solve(&[(2, 5)], 2), 5);
    }
}
