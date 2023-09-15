use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        xs: [[usize; n]; m],
    }

    let xs = xs
        .into_iter()
        .map(|xs| xs.into_iter().enumerate().fold(0, |y, (i, x)| y | x << i))
        .collect::<Vec<_>>();

    dbg!(&xs);
}
