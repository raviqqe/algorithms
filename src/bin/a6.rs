use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut xs: [usize; n],
        qs: [(Usize1, Usize1); m],
    }

    for i in 1..xs.len() {
        xs[i] += xs[i - 1];
    }

    for (i, j) in qs {
        println!("{}", xs[j] - (if i == 0 { 0 } else { xs[i - 1] }));
    }
}
