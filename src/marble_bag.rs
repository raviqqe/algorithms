//! Put Marbles in Bags.

use alloc::vec;
use alloc::vec::Vec;

pub fn marble_bag(row_count: i32) -> Vec<Vec<i32>> {
    let mut xs = vec![] as Vec<Vec<i32>>;

    for i in 0..row_count as usize {
        let len = i + 1;
        let mut ys = Vec::with_capacity(len);

        for j in 0..len {
            ys.push(if i == 0 {
                1
            } else {
                let xs = &xs[i - 1];

                (if j == 0 { 0 } else { xs[j - 1] }) + xs.get(j).copied().unwrap_or_default()
            });
        }

        xs.push(ys);
    }

    xs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
