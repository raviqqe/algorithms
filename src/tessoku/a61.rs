use std::{
    collections::HashSet,
    io::{stdin, Read},
};

pub fn run() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let xs = s
        .split('\n')
        .map(|s| {
            s.split(' ')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>();

    let [n, _] = &xs[0][..] else { return };

    let mut ys = vec![HashSet::new(); *n];

    for zs in &xs[1..] {
        let i = zs[0] - 1;
        let j = zs[1] - 1;

        ys[i].insert(j);
        ys[j].insert(i);
    }

    for (i, ys) in ys.iter().enumerate() {
        println!("{}: {:?}", i + 1, ys);
    }
}
