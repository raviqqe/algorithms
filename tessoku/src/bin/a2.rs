use proconio::input;

fn main() {
    input! {
        n: usize,
        y: usize,
        xs: [usize; n],
    }

    println!("{}", if xs.contains(&y) { "Yes" } else { "No" });
}
