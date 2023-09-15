use proconio::input;

fn main() {
    input! {
        n: usize,
        xs: [(f64, f64); n],
    }

    let mut dp = vec![vec![None; n]; 1 << n];

    for j in 0..n {
        dp[0][j] = Some(0);
    }

    for i in 1..1 << n {
        for j in 0..n {
            dp[i][j] = dp[i][j];
        }
    }

    println!("{}", 0);
}
