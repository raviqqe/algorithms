use std::{
    collections::HashSet,
    convert::identity,
    io::{Read, stdin},
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

    let mut ys = vec![HashSet::new(); *n];

    for xs in &xs[1..] {
        let i = xs[0] - 1;
        let j = xs[1] - 1;

        ys[i].insert(j);
        ys[j].insert(i);
    }

    let mut zs = vec![false; *n];

    visit(0, &ys, &mut zs);

    println!(
        "The graph is{} connected.",
        if zs.into_iter().all(identity) {
            ""
        } else {
            " not"
        }
    );
}

fn visit(x: usize, ys: &[HashSet<usize>], zs: &mut [bool]) {
    zs[x] = true;

    for &y in &ys[x] {
        if !zs[y] {
            visit(y, ys, zs);
        }
    }
}
