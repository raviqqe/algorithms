use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        xs: [[usize; n]; m],
    }

    let xs = xs
        .into_iter()
        .map(|xs| xs.into_iter().enumerate().fold(0, |y, (i, x)| y | x << i))
        .collect::<Vec<_>>();

    let mut dp = vec![None; 1 << n];
    dp[0] = Some(0);

    for i in 0..dp.len() {
        for &x in &xs {
            dp[i | x] = dp[i | x].into_iter().chain(dp[i].map(|x| x + 1)).min();
        }
    }

    println!("{}", dp.last().unwrap().unwrap_or(-1));
}
