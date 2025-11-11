//! LZSS compression.

const MIN_MATCH: usize = 3;

/// Compresses a byte array.
pub fn compress<const N: usize, const M: usize>(xs: &[u8]) -> Vec<u8> {
    let mut ys = vec![];
    let mut index = 0;

    while index < xs.len() {
        let mut best_len = 0;
        let mut best_offset = 0;

        for i in index.saturating_sub(N)..index {
            let mut length = 0;

            while length < M && index + length < xs.len() && xs[i + length] == xs[index + length] {
                length += 1;
            }

            if length >= MIN_MATCH && length > best_len {
                best_len = length;
                best_offset = index - i;
            }
        }

        if best_len >= MIN_MATCH {
            ys.push(1);
            ys.push((best_offset >> 8) as u8);
            ys.push(best_offset as u8);
            ys.push(best_len as u8);
            index += best_len;
        } else {
            ys.push(0);
            ys.push(xs[index]);
            index += 1;
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
