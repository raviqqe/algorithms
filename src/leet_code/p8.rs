pub fn atoi(s: String) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn simple() {
        assert_eq!(atoi("0".into()), 0);
    }
}
