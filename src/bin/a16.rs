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

    println!("{}", xs.last().copied().unwrap_or_default())
}
