use proconio::input;

fn main() {
    input! { n: usize, xs: [usize; n] }

    println!("{}", solve(&xs));
}

fn solve(xs: &[usize]) -> usize {
    let n = xs.len();
    let mut dp = vec![0; n];
    let mut ys = vec![usize::MAX; n];

    for i in 0..n {
        let x = xs[i];
        let l = if let Err(l) = ys[..i].binary_search(&x) {
            l
        } else {
            continue;
        };
        dp[i] = l;
        ys[l] = x;
    }

    dp.into_iter().max().unwrap() + 1
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
    }

    #[test]
    fn complex() {
        assert_eq!(solve(&[1, 2, 1, 2, 3, 4]), 4);
        assert_eq!(solve(&[2, 1, 2, 1, 3, 4]), 4);
        assert_eq!(solve(&[1, 2, 1, 2, 3, 4]), 4);
    }
}
