use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        xs: [[usize; n]; m],
    }

    dbg!(&xs);
}
