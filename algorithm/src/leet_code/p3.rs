use std::collections::HashSet;

pub fn length_of_longest_substring(s: String) -> i32 {
    let xs = s.as_bytes();

    let mut max = 0;
    let mut i = 0;
    let mut ys = HashSet::<u8>::new();

    for (j, &x) in xs.iter().enumerate() {
        if ys.contains(&x) {
            while ys.contains(&x) {
                ys.remove(&xs[i]);
                i += 1;
            }
        } else {
            max = max.max(j - i + 1);
        }

        ys.insert(x);
    }

    max as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn simple() {
        // spell-checker: disable
        assert_eq!(length_of_longest_substring("".into()), 0);
        assert_eq!(length_of_longest_substring(" ".into()), 1);
        assert_eq!(length_of_longest_substring("abcabcbb".into()), 3);
        assert_eq!(length_of_longest_substring("bbbbb".into()), 1);
        assert_eq!(length_of_longest_substring("abcabcdbb".into()), 4);
        assert_eq!(length_of_longest_substring("pwwkew".into()), 3);
        assert_eq!(length_of_longest_substring("dvdf".into()), 3);
    }
}
