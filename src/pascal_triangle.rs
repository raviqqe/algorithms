use alloc::vec;
use alloc::vec::Vec;

pub fn pascal_triangle(row_count: i32) -> Vec<Vec<i32>> {
    let mut xs = vec![] as Vec<Vec<i32>>;

    for i in 0..row_count as usize {
        xs.push(
            (0..i + 1)
                .map(|j| {
                    xs.get(i - 1)
                        .map(|xs| xs[j] + xs.get(j + 1).copied().unwrap_or_default())
                        .unwrap_or(1) as i32
                })
                .collect(),
        );
    }

    xs
}
