use proconio::input;

fn main() {
    input! {
        n: usize,
        xs: [(usize, usize); n],
    }

    println!("{}", solve(usize));
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
