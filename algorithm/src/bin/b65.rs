use std::{
    collections::HashSet,
    io::{stdin, Read},
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

    let [_, t] = &xs[0][..] else { return };

    let mut ys = vec![HashSet::new(); xs.len()];

    for xs in &xs[1..] {
        let i = xs[0] - 1;
        let j = xs[1] - 1;

        ys[i].insert(j);
        ys[j].insert(i);
    }

    let mut zs = vec![usize::MAX; xs.len()];

    visit(*t - 1, &ys, &mut zs);

    for z in zs {
        print!("{} ", z);
    }
}

fn visit(x: usize, ys: &[HashSet<usize>], zs: &mut [usize]) {
    zs[x] = 0;

    for &y in &ys[x] {
        if zs[y] == usize::MAX {
            visit(y, ys, zs);
            zs[x] = zs[x].max(zs[y] + 1);
        }
    }
}
