use algorithm::traveling_salesman_problem;
use proconio::input;

fn main() {
    input! {
        n: usize,
        xs: [(f64, f64); n],
    }

    println!("{}", traveling_salesman_problem::solve(&xs).0);
}
