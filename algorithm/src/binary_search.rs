use core::cmp::Ordering;

pub fn binary_search<T: Ord>(xs: &[T], y: &T) -> Result<usize, usize> {
    let mut left = 0;
    let mut right = xs.len();

    while left < right {
        let middle = (left + right) / 2;

        match xs[middle].cmp(y) {
            Ordering::Equal => return Ok(middle),
            Ordering::Less => left = middle + 1,
            Ordering::Greater => right = middle,
        }
    }

    Err(left)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_0_element() {
        assert_eq!(binary_search(&[], &0), Err(0));
    }

    #[test]
    fn search_1_element() {
        assert_eq!(binary_search(&[1], &0), Err(0));
        assert_eq!(binary_search(&[1], &1), Ok(0));
        assert_eq!(binary_search(&[1], &2), Err(1));
    }

    #[test]
    fn search_2_elements() {
        assert_eq!(binary_search(&[1, 3], &0), Err(0));
        assert_eq!(binary_search(&[1, 3], &1), Ok(0));
        assert_eq!(binary_search(&[1, 3], &2), Err(1));
        assert_eq!(binary_search(&[1, 3], &3), Ok(1));
        assert_eq!(binary_search(&[1, 3], &4), Err(2));
    }

    #[test]
    fn search_3_elements() {
        assert_eq!(binary_search(&[1, 3, 5], &0), Err(0));
        assert_eq!(binary_search(&[1, 3, 5], &1), Ok(0));
        assert_eq!(binary_search(&[1, 3, 5], &2), Err(1));
        assert_eq!(binary_search(&[1, 3, 5], &3), Ok(1));
        assert_eq!(binary_search(&[1, 3, 5], &4), Err(2));
        assert_eq!(binary_search(&[1, 3, 5], &5), Ok(2));
        assert_eq!(binary_search(&[1, 3, 5], &6), Err(3));
    }
}
