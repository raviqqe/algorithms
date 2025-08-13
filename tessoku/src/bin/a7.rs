use proconio::{input, marker::Usize1};

fn main() {
    input! {
        d: usize,
        n: usize,
        qs: [(Usize1, Usize1); n],
    }

    let mut xs = vec![0isize; d + 1];

    for (i, j) in qs {
        xs[i] += 1;
        xs[j + 1] -= 1;
    }

    let mut y = 0;

    for x in &xs[..d] {
        y += x;

        println!("{y}");
    }
}
