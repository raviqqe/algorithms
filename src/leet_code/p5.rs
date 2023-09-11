pub fn longest_palindrome(s: String) -> String {
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn simple() {
        assert_eq!(longest_palindrome("".into()), "".to_string());
    }
}
