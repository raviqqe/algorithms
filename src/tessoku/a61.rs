use std::io::{stdin, Read};

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

    let [n, m] = &xs[0][..] else { return };

    let mut ys = vec![vec![]; n];
}
