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

    let xs = &xs[1][..];

    let mut ys = vec![0; xs.len() + 1];

    for (i, x) in xs.iter().enumerate().rev() {
        ys[x - 1] += ys[i + 1] + 1;
    }

    for y in ys {
        print!("{y} ");
    }
}
