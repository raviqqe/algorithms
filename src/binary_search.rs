pub fn binary_search<T: Ord>(xs: &[T], x: T) -> Result<usize, usize> {
    loop {
        let index = xs.len() / 2;

        xs[index] < x;
    }

    Ok(index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search() {
        assert_eq!(binary_search(&[], 0), Err(0));
        assert_eq!(binary_search(&[1], 0), Err(0));
        assert_eq!(binary_search(&[1], 1), Ok(0));
        assert_eq!(binary_search(&[1], 2), Err(1));
        assert_eq!(binary_search(&[1, 3, 5], 0), Err(0));
        assert_eq!(binary_search(&[1, 3, 5], 1), Ok(0));
    }
}
