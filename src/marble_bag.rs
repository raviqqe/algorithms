//! Put Marbles in Bags.

pub fn marble_bag(weights: Vec<i32>, bag_count: i32) -> i64 {
    let mut xs = Vec::with_capacity(weights.len() - 1);
    xs.extend((0..weights.len() - 1).map(|i| (weights[i] + weights[i + 1]) as i64));
    xs.sort();

    xs[xs.len() + 1 - bag_count as usize..]
        .iter()
        .copied()
        .sum::<i64>()
        - xs[..bag_count as usize - 1].iter().copied().sum::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn small_input() {
        assert_eq!(marble_bag(vec![1], 1), 0);
        assert_eq!(marble_bag(vec![1, 1], 1), 0);
        assert_eq!(marble_bag(vec![1, 1, 1], 2), 0);
        assert_eq!(marble_bag(vec![1, 3, 5, 1], 2), 4);
    }

    #[test]
    fn large_input() {
        assert_eq!(
            marble_bag(
                vec![
                    1, 5, 64, 42, 40, 60, 7, 54, 25, 71, 11, 17, 2, 52, 54, 41, 1, 28, 2, 1, 68,
                    13, 25, 16, 26, 39, 36, 24, 13, 61, 51, 11, 3, 36, 58, 15
                ],
                17
            ),
            850
        );
    }
}
