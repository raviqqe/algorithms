use std::collections::HashSet;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut max = 0;
    let mut cs = HashSet::<char>::new();
    let mut len = 0;

    for c in s.chars() {
        if cs.contains(&c) {
            max = max.max(len);
            cs = [c].into_iter().collect();
            len = 1;
        } else {
            len += 1;
            cs.insert(c);
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn simple() {
        assert_eq!(length_of_longest_substring("abcabcbb".into()), 3);
        assert_eq!(length_of_longest_substring("bbbbb".into()), 1);
        assert_eq!(length_of_longest_substring("abcabcdbb".into()), 4);
        assert_eq!(length_of_longest_substring("pwwkew".into()), 3);
    }
}
