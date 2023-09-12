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

    let [n, _] = &xs[0][..] else { return };

    let mut ys = vec![HashSet::new(); *n];

    for xs in &xs[1..] {
        let i = xs[0];
        let j = xs[1];

        ys[i - 1].insert(j);
        ys[j - 1].insert(i);
    }

    for (i, ys) in ys.iter().enumerate() {
        println!("{}: {:?}", i + 1, ys);
    }
}
