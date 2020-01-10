fn hamming(a: &[u8], b: &[u8]) -> u32 {
    if a.len() != b.len() {
        panic!("strings must be same length to compute edit distance");
    }
    let mut result = 0;
    for i in 0..a.len() {
        result += (a[i] ^ b[i]).count_ones();
    }
    result
}

fn rank_keysizes(bytes: &[u8]) -> Vec<u32> {
    let max = std::cmp::min(40, bytes.len() / 2);
    let mut result = Vec::with_capacity(max - 1);
    for i in 2u32..=(max as u32) {
        let score = normalized_hamming_for_keysize(bytes, i);
        if let Some(g) = GoodFloat::new(score) {
            result.push((i, g));
        }
    }

    result.sort_by_key(|(_,score)| *score);

    result.into_iter().map(|(keysize,_)| keysize).collect()
}

#[derive(Debug,PartialEq,Eq,Clone,Copy,PartialOrd)]
pub struct GoodFloat(f64);
impl GoodFloat {
    pub fn new(n: f64) -> Option<GoodFloat> {
        if n.is_nan() {
            None
        } else {
            Some(GoodFloat(n))
        }
    }
}

impl Ord for GoodFloat {
    fn cmp(&self, other: &GoodFloat) -> std::cmp::Ordering {
        self.0.partial_cmp(other.0).unwrap()
    }
 }

fn normalized_hamming_for_keysize(bytes: &[u8], keysize: u32) -> f64 {
    let dist = hamming(bytes[..keysize], bytes[keysize..keysize*2]);
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
