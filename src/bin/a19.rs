use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        xs: [(usize, usize); n],
    }

    let mut dp = vec![vec![None; w + 1]; n + 1];
    dp[0][0] = Some(0);

    for i in 1..=n {
        for j in 0..=w {
            let (w, v) = xs[i - 1];

            dp[i][j] = dp[i - 1][j].max(if j >= w {
                dp[i - 1][j - w].map(|x| x + v)
            } else {
                None
            });
        }
    }

    println!("{}", dp.iter().flatten().flatten().max().unwrap());
}
