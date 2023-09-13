use proconio::input;

fn main() {
    input! {
        n: usize,
        ls: [usize; n - 1],
        ms: [usize; n - 2],
    }

    let mut xs = vec![[0; 2]; n - 1];

    for (i, &l) in ls.iter().enumerate() {
        xs[i][0] = l;
    }

    for (i, &m) in ms.iter().enumerate() {
        xs[i][1] = m;
    }

    dbg!(&xs);
}
