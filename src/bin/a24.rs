use proconio::input;

fn main() {
    input! { n: usize, xs: [usize; n] }

    println!("{}", solve(&xs));
}

fn solve(xs: &[usize]) -> usize {
    let n = xs.len();
    let mut dp = vec![(1, 0); n];

    for i in 1..n {
        let (x, j) = dp[i - 1];

        dp[i] = if xs[j] < xs[i] { (x + 1, i) } else { (x, j) };
    }

    dp.last().unwrap().0
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn simple() {
        assert_eq!(solve(&[1]), 1);
        assert_eq!(solve(&[1, 2]), 2);
        assert_eq!(solve(&[1, 2, 3]), 3);
        assert_eq!(solve(&[1, 2, 0, 3]), 3);
        assert_eq!(solve(&[1, 0, 2, 3]), 3);
        assert_eq!(solve(&[1, 2, 3, 0]), 3);
        assert_eq!(solve(&[3, 2, 1]), 1);
        assert_eq!(solve(&[1, 2, 1, 3, 4]), 4);
        assert_eq!(solve(&[2, 1, 2, 1, 3, 4]), 4);
    }
}
