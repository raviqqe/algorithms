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

    let mut ys = vec![xs.len() - 1];

    while ys[ys.len() - 1] > 0 {
        let i = ys[ys.len() - 1];
        ys.push(i - if xs[i] == xs[i - 1] + ls[i - 1] { 1 } else { 2 });
    }

    println!("{}", ys.len());

    for i in ys.into_iter().rev() {
        print!("{} ", i + 1);
    }
}
