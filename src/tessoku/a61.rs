use std::io::{stdin, Read};

pub fn run() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let xs = s
        .split('\n')
        .map(|s| s.split(' ').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let [n, m] = &xs[0];
}
