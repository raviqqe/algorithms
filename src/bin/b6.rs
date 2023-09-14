use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        xs: [usize; n],
        m: usize,
        qs: [(Usize1, Usize1); m],
    }

    let mut ys = vec![(0, 0); n];

    for i in 0..xs.len() {
        let x = xs[i];
        let (w, l) = if i > 0 { ys[i - 1] } else { (0, 0) };

        ys[i] = (w + x, l + 1 - x);
    }

    for (i, j) in qs {
        let (w0, l0) = if i > 0 { ys[i - 1] } else { (0, 0) };
        let (w1, l1) = ys[j];
        let w = w1 - w0;
        let l = l1 - l0;

        println!(
            "{}",
            if w == l {
                "draw"
            } else if w < l {
                "lose"
            } else {
                "win"
            }
        );
    }
}
