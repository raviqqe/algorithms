//! Put Marbles in Bags.

pub fn marble_bag(weights: Vec<i32>, bag_count: i32) -> i64 {
    let mut min = i64::MAX;
    let mut max = i64::MIN;

    take_marble(
        0,
        weights[0],
        &weights,
        bag_count as usize,
        &mut min,
        &mut max,
    );

    max - min
}

fn take_marble(
    sum: i64,
    first_weight: i32,
    weights: &[i32],
    bag_count: usize,
    min: &mut i64,
    max: &mut i64,
) {
    if weights.len() == 1 {
        debug_assert!(bag_count == 1);

        let sum = sum + (first_weight as i64 + *weights.last().unwrap() as i64);

        *min = sum.min(*min);
        *max = sum.max(*max);

        return;
    }

    if weights.len() > bag_count {
        take_marble(sum, first_weight, &weights[1..], bag_count, min, max);
    }

    if bag_count > 1 {
        take_marble(
            sum + first_weight as i64 + weights[0] as i64,
            weights[1],
            &weights[1..],
            bag_count - 1,
            min,
            max,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test() {
        assert_eq!(marble_bag(vec![1], 1), 0);
        assert_eq!(marble_bag(vec![1, 1], 1), 0);
        assert_eq!(marble_bag(vec![1, 1, 1], 2), 0);
        assert_eq!(marble_bag(vec![1, 3, 5, 1], 2), 4);
    }
}
