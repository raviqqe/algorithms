use proconio::{input, marker::Usize1};

fn main() {
    input! {
        d: usize,
        n: usize,
        qs: [(Usize1, Usize1); n],
    }

    let mut xs = vec![0; d];
    let mut ys = vec![0; d];

    for (i, j) in qs {
        xs[i] += 1;
        ys[j] += 1;
    }

    let mut zs = vec![0; d];
    zs[0] = xs[0];

    for i in 1..d {
        zs[i] = zs[i - 1] + xs[i] - ys[i - 1];
    }

    for z in zs {
        println!("{}", z);
    }
}
