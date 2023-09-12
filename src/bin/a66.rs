use std::io::{stdin, Read};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let xs = s
        .split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.split(' ')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>();

    let [n, _] = &xs[0][..] else { return };

    let mut ys = vec![None; *n];

    for xs in &xs[1..] {
        let q = xs[0];
        let i = xs[1] - 1;
        let j = xs[2] - 1;

        if q == 1 {
            union(i, j, &mut ys);
        } else {
            println!(
                "{} ",
                if root(i, &ys) == root(j, &ys) {
                    "Yes"
                } else {
                    "No"
                }
            );
        }
    }
}

fn union(x: usize, y: usize, zs: &mut [Option<usize>]) {
    match (zs[x], zs[y]) {
        (None, None) | (None, Some(_)) => zs[x] = Some(y),
        (Some(_), None) => union(y, x, zs),
        (Some(_), Some(_)) => zs[root(x, zs)] = Some(root(y, zs)),
    }
}

fn root(x: usize, ys: &[Option<usize>]) -> usize {
    let y = ys[x];

    if let Some(y) = y {
        root(y, ys)
    } else {
        x
    }
}
