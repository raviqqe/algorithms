use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        xs: [(usize, usize); n],
    }

    println!("{:?}", solve(&xs, m));
}

fn solve(xs: &[(usize, usize)], m: usize) -> usize {
    let mut dp = vec![vec![0; m + 1]; xs.len() + 1];

    for i in 0..xs.len() {}

    dp[xs.len()].iter().copied().max().unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!(solve(&[(2, 5)], 2), 5);
    }
}
