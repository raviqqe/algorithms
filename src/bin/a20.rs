use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }

    let s = s.as_bytes();
    let t = t.as_bytes();

    let mut dp = vec![vec![0; t.len()]; s.len()];

    for i in 0..s.len() {
        for j in 0..t.len() {
            dp[i][j] = if s[i] == t[j] {
                (if i > 0 && j > 0 { dp[i - 1][j - 1] } else { 0 }) + 1
            } else {
                (if j > 0 { dp[i][j - 1] } else { 0 }).max(if i > 0 { dp[i - 1][j] } else { 0 })
            };
        }
    }

    println!("{}", dp.iter().flatten().max().unwrap());
}
