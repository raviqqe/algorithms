use proconio::input;
use std::convert::identity;

fn main() {
    input! { n: usize, xs: [usize; n] }

    println!("{}", solve(&xs));
}

fn solve(xs: &[usize]) -> usize {
    let n = xs.len();
    let mut ys = vec![usize::MAX; n];

    for i in 0..n {
        let x = xs[i];
        let l = ys[..i].binary_search(&x).map_or_else(identity, identity);

        ys[l] = x;
    }

    ys.into_iter()
        .enumerate()
        .rev()
        .find(|(_, l)| *l < usize::MAX)
        .unwrap()
        .0
        + 1
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
