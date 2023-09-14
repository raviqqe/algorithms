use proconio::{input, marker::Usize1};

const A: usize = 100;
const B: usize = 150;

fn main() {
    input! {
        n: usize,
        xs: [Usize1; n - 1],
        ys: [Usize1; n - 1],
    }

    println!("{}", solve(&xs, &ys));
}

fn solve(xs: &[usize], ys: &[usize]) -> usize {
    let n = xs.len() + 1;
    let mut dp = vec![None; n];
    dp[0] = Some(0);

    for i in 0..n - 1 {
        for (j, v) in [(xs[i], A), (ys[i], B)] {
            dp[j] = dp[j].max(dp[i].map(|x| x + v));
        }
    }

    dp.last().unwrap().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn simple() {
        assert_eq!(solve(&[1], &[1]), 150);
        assert_eq!(solve(&[1, 2], &[2, 2]), 250);
        assert_eq!(solve(&[1, 2], &[1, 2]), 300);
        assert_eq!(solve(&[1, 3, 3, 6, 5, 6], &[2, 4, 5, 6, 6, 6]), 500)
    }

    #[test]
    fn unreachable() {
        assert_eq!(solve(&[1, 2, 3, 4], &[4, 2, 3, 4]), 550);
    }
}
