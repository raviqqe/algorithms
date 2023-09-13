use std::{
    io::{stdin, Read},
    mem::swap,
};

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

    let mut zs = vec![None; *n];
    let mut ns = vec![1; *n];

    for xs in &xs[1..] {
        let q = xs[0];
        let i = xs[1] - 1;
        let j = xs[2] - 1;

        if q == 1 {
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

fn union(mut x: usize, mut y: usize, zs: &mut [Option<usize>], ns: &mut [usize]) {
    if ns[x] > ns[y] {
        swap(&mut x, &mut y);
    }

    let x = root(x, zs);
    let y = root(y, zs);

    if x != y {
        zs[x] = Some(y);
        ns[y] += ns[x];
    }
}

fn root(mut x: usize, ys: &[Option<usize>]) -> usize {
    while let Some(y) = ys[x] {
        x = y;
    }

    x
}
