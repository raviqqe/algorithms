use proconio::input;

fn main() {
    input! {
      n: usize,
      m: usize,
      xs: [usize; n],
    }

    println!("{}", solve(&xs, m));
}

fn solve(xs: &[usize], m: usize) -> bool {
    let mut dp = vec![vec![false; m + 1]; xs.len()];

    for i in 1..=xs.len() {
        foo;
    }

    dp[xs.len()][m]
}
