use proconio::input;
use std::convert::identity;

fn main() {
    input! {
        n: usize,
        y: usize,
        xs: [usize; n],
    }

    let mut dp = vec![vec![false; y + 1]; n + 1];
    dp[0][0] = true;

    for i in 1..=n {
        for j in 0..=y {
            dp[i][j] = dp[i - 1][j] || j >= xs[i - 1] && dp[i - 1][j - xs[i - 1]];
        }
    }

    println!(
        "{}",
        if dp
            .into_iter()
            .flat_map(|dp| dp.last().copied())
            .any(identity)
        {
            "Yes"
        } else {
            "No"
        }
    )
}
