//! LZSS compression.

const MIN_MATCH: usize = 2;

/// Compresses a byte array.
pub fn compress<const W: usize, const N: usize>(xs: &[u8]) -> Vec<u8> {
    let mut ys = vec![];
    let mut i = 0;

    while i < xs.len() {
        let (n, m) = (i.saturating_sub(W)..i)
            .map(|j| {
                let mut n = 0;

                while n < N && xs.get(i + n) == xs.get(j + n) {
                    n += 1;
                }

                (i - j, n)
            })
            .max_by_key(|(_, m)| *m)
            .unwrap_or_default();

        if m > MIN_MATCH {
            ys.extend([(n as u8) << 1 | 1, m as u8]);

            i += m;
        } else {
            ys.push(xs[i] << 1);

            i += 1;
        }
    }

    ys
}

/// Decompresses a byte array.
pub fn decompress(xs: &[u8]) -> Vec<u8> {
    let mut ys = vec![];
    let mut i = 0;

    while let Some(&x) = xs.get(i) {
        if x.is_multiple_of(2) {
            ys.push(x >> 1);
        } else {
            for _ in 0..xs[i + 1] {
                ys.push(ys[ys.len() - (x >> 1) as usize]);
            }
        }

        i += 1 + x as usize % 2;
    }

    ys
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn repetitions() {
        let data = b"ABABABABABABABABABABA123123123123";
        let compressed = compress::<64, 256>(data);

        assert!(compressed.len() < data.len());
        assert_eq!(decompress(&compressed), data);
    }

    proptest! {
      #[test]
      fn random(data: Vec<u8>) {
        let data = data.into_iter().map(|byte| byte >> 1).collect::<Vec<_>>();
        let compressed = compress::<64, 256>(&data);

        assert_eq!(decompress(&compressed), data);
      }
    }
}
