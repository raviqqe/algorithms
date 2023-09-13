use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }

    let s = s.as_bytes();
    let t = t.as_bytes();

    let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];

    for i in 1..=s.len() {
        for j in 1..=t.len() {
            dp[i][j] = if s[i - 1] == t[j - 1] {
                dp[i - 1][j - 1] + 1
            } else {
                dp[i][j - 1].max(dp[i - 1][j])
            };
        }
    }

    println!("{}", dp[s.len()][t.len()]);
}
