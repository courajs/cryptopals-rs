use ordered_float::OrderedFloat;

fn hamming(a: &[u8], b: &[u8]) -> usize {
    if a.len() != b.len() {
        panic!("strings must be same length to compute edit distance");
    }
    let mut result = 0;
    for i in 0..a.len() {
        result += (a[i] ^ b[i]).count_ones() as usize;
    }
    result
}

fn rank_keysizes(bytes: &[u8]) -> Vec<usize> {
    let max = std::cmp::min(40, bytes.len() / 2);
    let mut result: Vec<usize> = (2usize..=max).collect();
    result.sort_by_cached_key(|keysize| {
        OrderedFloat(normalized_hamming_for_keysize(bytes, *keysize as usize))
    });
    result
}

fn normalized_hamming_for_keysize(bytes: &[u8], keysize: usize) -> f64 {
    let dist = hamming(&bytes[..keysize], &bytes[keysize..keysize*2]);
    dist as f64 / keysize as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming() {
        let a = b"this is a test";
        let b = b"wokka wokka!!!";

        assert_eq!(hamming(a, b), 37);
    }

    #[test]
    fn guess_keysize() {
        use crate::set1::c5;
        let cleartext = b"hello I believe in cars and technology";
        let vignere = c5::repeating_key_xor(cleartext, b"yes");
        let keysizes = rank_keysizes(&vignere);

        assert_eq!(keysizes[0], 3);
    }
}
