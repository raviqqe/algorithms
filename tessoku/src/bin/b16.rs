use proconio::input;

fn main() {
    input! {
        n: usize,
        hs: [isize; n],
    }

    let mut xs = vec![0; n];

    let cost = |i, j| {
        let c: isize = hs[i] - hs[j];
        c.unsigned_abs()
    };

    for i in 1..xs.len() {
        xs[i] = (xs[i - 1] + cost(i - 1, i)).min(if i == 1 {
            usize::MAX
        } else {
            xs[i - 2] + cost(i - 2, i)
        });
    }

    println!("{}", xs.last().unwrap())
}
