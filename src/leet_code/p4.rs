use std::convert::identity;

pub fn find_median_sorted_arrays(xs: Vec<i32>, ys: Vec<i32>) -> f64 {
    let m = (xs.len() + ys.len()) / 2;
    let i = xs.len() / 2;
    let j = ys.binary_search(&xs[i]).unwrap_or_else(identity);

    while i + j != m {
        if i + j > m {
            todo!();
        } else {
            todo!();
        }
    }

    xs[i] as f64 + ys[j] as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!(find_median_sorted_arrays(vec![1], vec![2]), 1.5);
    }
}
