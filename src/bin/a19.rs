use proconio::input;

const V: usize = 1000;

fn main() {
    input! {
        n: usize,
        w: usize,
        xs: [(usize, usize); n],
    }

    let m = V * n;
    let mut dp = vec![vec![None; m + 1]; n + 1];
    dp[0][0] = Some(0);

    for i in 1..=n {
        for j in 0..m + 1 {
            let (w, v) = xs[i - 1];

            dp[i][j] = match (
                dp[i - 1][j],
                if j >= v {
                    dp[i - 1][j - v].map(|x| x + w)
                } else {
                    None
                },
            ) {
                (Some(w1), Some(w2)) => Some(w1.min(w2)),
                (Some(w), None) | (None, Some(w)) => Some(w),
                (None, None) => None,
            };
        }
    }

    println!(
        "{}",
        dp.last()
            .unwrap()
            .iter()
            .enumerate()
            .rev()
            .find_map(|(i, x)| if let Some(x) = x {
                if *x <= w {
                    Some(i)
                } else {
                    None
                }
            } else {
                None
            })
            .unwrap()
    );
}
