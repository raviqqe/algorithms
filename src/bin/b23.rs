use proconio::input;

fn main() {
    input! {
        n: usize,
        xs: [(f64, f64); n],
    }

    let mut dp = vec![vec![foo]; 1 << n];

    println!("{}", 0);
}
