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

    for i in 0..d {
        zs[i] = (if i == 0 { 0 } else { zs[i - 1] }) + xs[i] - (if i == 0 { 0 } else { ys[i - 1] });
    }

    for z in zs {
        println!("{}", z);
    }
}
