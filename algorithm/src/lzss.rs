//! LZSS compression.

const MIN_MATCH: usize = 3;

/// Compresses a byte array.
pub fn compress<const N: usize, const M: usize>(input: &[u8]) -> Vec<u8> {
    let mut output = vec![];
    let mut index = 0;

    while index < input.len() {
        let mut best_len = 0;
        let mut best_offset = 0;

        for i in index.saturating_sub(N)..index {
            let mut length = 0;

            while length < M
                && index + length < input.len()
                && input[i + length] == input[index + length]
            {
                length += 1;
            }

            if length >= MIN_MATCH && length > best_len {
                best_len = length;
                best_offset = index - i;
            }
        }

        if best_len >= MIN_MATCH {
            output.push(1);
            output.push((best_offset >> 8) as u8);
            output.push(best_offset as u8);
            output.push(best_len as u8);
            index += best_len;
        } else {
            output.push(0);
            output.push(input[index]);
            index += 1;
        }
    }

    output
}

/// Decompresses a byte array.
pub fn decompress(input: &[u8]) -> Vec<u8> {
    let mut output = vec![];
    let mut index = 0;

    while index < input.len() {
        let x = input[index];
        index += 1;

        if x.is_multiple_of(2) {
            output.push(input[index]);
            index += 1;
        } else {
            let offset = ((input[index] as usize) << 8) | (input[index + 1] as usize);
            let length = input[index + 2] as usize;
            index += 3;

            let start = output.len() - offset;
            for i in 0..length {
                let b = output[start + i];
                output.push(b);
            }
        }
    }

    output
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
