pub fn is_match(s: String, p: String) -> bool {
    let mut xs = s.chars();
    let mut ys = p.chars();
    let mut y = ys.next();

    while y.is_some() {
        let z = ys.next();

        match z {
            Some('*') => todo!(),
            Some(_) | None => {
                let x = xs.next();

                if y != Some('.') && x != y {
                    return false;
                }
            }
        }

        y = z;
    }

    xs.next().is_none()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn simple() {
        assert_eq!(is_match("".into(), "".into()), true);
        assert_eq!(is_match("a".into(), "".into()), false);
        assert_eq!(is_match("".into(), "a".into()), false);
        assert_eq!(is_match("a".into(), "a".into()), true);
    }
}
