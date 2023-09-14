use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut xs: [usize; n],
        qs: [(usize, usize); m],
    }

    for i in 1..xs.len() {
        xs[i] += xs[i - 1];
    }

    for (i, j) in qs {
        println!(xs[j] - xs[i]);
    }
}
