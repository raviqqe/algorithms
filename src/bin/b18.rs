use proconio::input;

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

    let mut j = y;
    let mut ys = vec![];

    for i in (1..=n).rev() {
        if dp[i][j] {
            ys.push(i);
            j -= xs[i - 1];

            if j == 0 {
                break;
            }
        }
    }

    println!("{}", ys.len());

    for y in ys.iter().rev() {
        print!("{} ", y);
    }
}
