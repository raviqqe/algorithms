use proconio::input;

fn main() {
    input! { t: usize, n: usize, xs: [(usize, usize); n] }

    let mut ys = vec![0isize; t + 1];

    for (i, j) in xs {
        ys[i] += 1;
        ys[j] -= 1;
    }

    let mut z = 0;

    for y in &ys[..t] {
        z += y;

        println!("{z}");
    }
}
