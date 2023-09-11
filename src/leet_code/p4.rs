pub fn find_median_sorted_arrays(xs: Vec<i32>, ys: Vec<i32>) -> f64 {
    let mut zs = xs.iter().chain(&ys).copied().collect::<Vec<_>>();

    zs.sort();

    (zs[(zs.len() - 1) / 2] as f64 + zs[zs.len() / 2] as f64) / 2.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!(find_median_sorted_arrays(vec![1], vec![2]), 1.5);
        assert_eq!(find_median_sorted_arrays(vec![1], vec![2, 3]), 2.0);
        assert_eq!(find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
        assert_eq!(find_median_sorted_arrays(vec![1, 2, 3], vec![4]), 2.5);
        assert_eq!(find_median_sorted_arrays(vec![1], vec![2, 3, 4]), 2.5);
        assert_eq!(find_median_sorted_arrays(vec![1, 2, 3], vec![4, 5]), 3.0);
        assert_eq!(find_median_sorted_arrays(vec![1, 2], vec![3, 4, 5]), 3.0);
    }
}
