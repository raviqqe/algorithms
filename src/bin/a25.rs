use proconio::{input, marker::Bytes};

fn main() {
    input! {
        h: usize,
        w: usize,
        xs: [Bytes; h],
    }

    let mut ys = vec![vec![0usize; w]; h];
    ys[0][0] = 1;

    for i in 0..h {
        for j in 0..w {
            ys[i][j] = if xs[i][j] == b'#' {
                0
            } else if i == 0 && j == 0 {
                1
            } else {
                [
                    if i > 0 { Some(ys[i - 1][j]) } else { None },
                    if j > 0 { Some(ys[i][j - 1]) } else { None },
                ]
                .into_iter()
                .flatten()
                .sum()
            };
        }
    }

    println!("{}", ys.last().unwrap().last().unwrap());
}
