use proconio::input;

fn main() {
    input! {
        n: usize,
        xs: [usize; n - 1],
        ys: [usize; n - 2],
    }

    println!("{}", solve(&xs, &ys));
}

fn solve(xs: &[usize], ys: &[usize]) -> usize {
    let n = xs.len() + 1;
    let mut dp = vec![0; n];

    for i in 0..n {
        if i < n - 1 {
            dp[i] = 0;
        } else if i < n - 2 {
            dp[i + 2] = foo;
        }
    }

    dp[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!(solve(&[1, 2], &[0]), 0);
    }
}
