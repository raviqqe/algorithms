use alloc::vec;
use alloc::vec::Vec;

pub fn pascal_triangle(row_count: i32) -> Vec<Vec<i32>> {
    let mut xs = vec![] as Vec<Vec<i32>>;

    for i in 0..row_count as usize {
        xs.push(
            (0..i + 1)
                .map(|j| {
                    if let Some(xs) = xs.get(i.saturating_sub(1)) {
                        (if j == 0 { 0 } else { xs[j - 1] })
                            + xs.get(j).copied().unwrap_or_default()
                    } else {
                        1
                    }
                })
                .collect(),
        );
    }

    xs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(pascal_triangle(0), vec![] as Vec<Vec<_>>);
        assert_eq!(pascal_triangle(1), vec![vec![1]]);
        assert_eq!(pascal_triangle(2), vec![vec![1], vec![1, 1]]);
        assert_eq!(pascal_triangle(3), vec![vec![1], vec![1, 1], vec![1, 2, 1]]);
        assert_eq!(
            pascal_triangle(4),
            vec![vec![1], vec![1, 1], vec![1, 2, 1], vec![1, 3, 3, 1]]
        );
    }
}
