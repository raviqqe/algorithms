use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut xs = HashMap::<i32, usize>::with_capacity(nums.len());

    for (i, &x) in nums.iter().enumerate() {
        if let Some(j) = xs.get(&(target - x)) {
            return vec![*j as i32, i as i32];
        }

        xs.insert(x, i);
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
