use std::{
    collections::{HashMap, VecDeque},
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

    let [n, _] = &xs[0][..] else { return };

    let mut ys = vec![HashMap::new(); *n];

    for xs in &xs[1..] {
        let i = xs[0] - 1;
        let j = xs[1] - 1;
        let w = xs[2];

        ys[i].insert(j, w);
        ys[j].insert(i, w);
    }

    let mut zs = vec![-1isize; *n];

    distance(&ys, &mut zs);

    for z in zs {
        println!("{}", z);
    }
}

fn distance(ys: &[HashMap<usize, usize>], zs: &mut [isize]) {
    let mut q = VecDeque::new();

    q.push_back(0);
    zs[0] = 0;

    while let Some(x) = q.pop_front() {
        for (&y, &w) in &ys[x] {
            let z = zs[x] + w as isize;

            zs[y] = if zs[y] < 0 {
                q.push_back(y);
                z
            } else {
                zs[y].min(z)
            };
        }
    }
}
