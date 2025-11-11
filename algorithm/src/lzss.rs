//! LZSS compression.

const MIN_MATCH: usize = 3;

/// Compresses a byte array.
pub fn compress<const W: usize, const L: usize>(xs: &[u8]) -> Vec<u8> {
    let mut ys = vec![];
    let mut i = 0;

    while i < xs.len() {
        let mut n = 0;
        let mut m = 0;

        for j in i.saturating_sub(W)..i {
            let mut length = 0;

            while length < L && i + length < xs.len() && xs[j + length] == xs[i + length] {
                length += 1;
            }

            if length >= MIN_MATCH && length > m {
                n = i - j;
                m = length;
            }
        }

        if m >= MIN_MATCH {
            ys.extend([(n as u8) << 1 & 1, m as u8]);

            i += m;
        } else {
            ys.push(xs[i]);

            i += 1;
        }
    }

    ys
}

/// Decompresses a byte array.
pub fn decompress(xs: &[u8]) -> Vec<u8> {
    let mut ys = vec![];
    let mut i = 0;

    while i < xs.len() {
        let x = xs[i];

        if x.is_multiple_of(2) {
            ys.push(x);

            i += 1;
        } else {
            for j in 0..xs[i] {
                ys.push(ys[ys.len() - (x >> 1) as usize + j as usize]);
            }

            i += 2;
        }
    }

    ys
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let data = b"ABABABABABABABABABABA123123123123";
        let compressed = compress::<64, 4>(data);

        assert_eq!(decompress(&compressed), data);
    }
}
