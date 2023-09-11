pub fn length_of_longest_substring(s: String) -> i32 {
    let xs = s.as_bytes();

    let mut max = 0;
    let mut i = 0;
    let mut len = 0;

    for (j, x) in xs.iter().enumerate() {
        if xs[i..j].contains(x) {
            max = max.max(len);
            i = j;
            len = 1;
        } else {
            len += 1;
        }
    }

    max.max(len)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn simple() {
        assert_eq!(length_of_longest_substring("".into()), 0);
        assert_eq!(length_of_longest_substring(" ".into()), 1);
        assert_eq!(length_of_longest_substring("abcabcbb".into()), 3);
        assert_eq!(length_of_longest_substring("bbbbb".into()), 1);
        assert_eq!(length_of_longest_substring("abcabcdbb".into()), 4);
        assert_eq!(length_of_longest_substring("pwwkew".into()), 3);
        assert_eq!(length_of_longest_substring("dvdf".into()), 3);
    }
}
