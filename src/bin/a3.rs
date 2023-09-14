use proconio::input;

fn main() {
    input! {
        n: usize,
        xs: [usize; n],
    }

    println!("{}", if solve(&xs) { "Yes" } else { "No" });
}

fn solve(xs: &[usize]) -> bool {
    for i in 0..xs.len() {
        for j in 0..xs.len() {
            for k in 0..xs.len() {
                if i != j && j != k && k != i && xs[i] + xs[j] + xs[k] == 1000 {
                    return true;
                }
            }
        }
    }

    false
}
