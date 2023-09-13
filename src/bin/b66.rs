use proconio::{input, marker::Usize1};
use std::mem::swap;

fn main() {
    input! {
        n: usize,
        m: usize,
        es: [(Usize1, Usize1, Usize1); m],
        qs: [(Usize1, Usize1, Usize1)],
    }

    let mut zs = vec![None; n];
    let mut ns = vec![1; n];

    for (q, i, j) in qs {
        if q == 0 {
            union(i, j, &mut zs, &mut ns);
        } else {
            println!(
                "{} ",
                if root(i, &zs) == root(j, &zs) {
                    "Yes"
                } else {
                    "No"
                }
            );
        }
    }
}

fn union(x: usize, y: usize, zs: &mut [Option<usize>], ns: &mut [usize]) {
    let mut x = root(x, zs);
    let mut y = root(y, zs);

    if x == y {
        return;
    } else if ns[x] > ns[y] {
        swap(&mut x, &mut y);
    }

    zs[x] = Some(y);
    ns[y] += ns[x];
}

fn root(mut x: usize, ys: &[Option<usize>]) -> usize {
    while let Some(y) = ys[x] {
        x = y;
    }

    x
}
