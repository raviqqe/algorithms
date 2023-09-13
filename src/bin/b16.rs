use proconio::input;

fn main() {
    input! {
        n: usize,
        hs: [usize; n],
    }

    let mut xs = vec![0; n];

    for i in 0..xs.len() {
        xs[i] = (if i == 0 { 0 } else { xs[i - 1] } + ls[i]).min(if i == 0 {
            usize::MAX
        } else {
            (if i == 1 { 0 } else { xs[i - 2] }) + ms[i - 1]
        });
    }

    println!("{}", xs.last().copied().unwrap_or_default())
}
