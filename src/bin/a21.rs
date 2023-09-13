use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        xs: [(Usize1, usize); n],
    }

    println!("{}", solve(&xs));
}

fn solve(xs: &[(usize, usize)]) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn simple() {
        assert_eq!();
    }
}
