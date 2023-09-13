use proconio::input;

fn main() {
    input! {
        n: usize,
        ls: [usize; n - 1],
        ms: [usize; n - 2],
    }

    let mut xs = vec![0; n];

    for i in 1..xs.len() {
        xs[i] = (xs[i - 1] + ls[i - 1]).min(if i == 1 {
            usize::MAX
        } else {
            xs[i - 2] + ms[i - 2]
        });
    }

    let mut ys = vec![];

    let mut i = xs.len() - 1;

    while i > 0 {
        i = i - if xs[i] == xs[i - 1] + ls[i - 1] { 1 } else { 2 };
        ys.push(i);
    }

    println!("{}", ys.len() + 1);

    for i in ys.into_iter().rev().chain([xs.len() - 1]) {
        print!("{} ", i + 1);
    }
}
