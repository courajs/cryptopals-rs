use std::collections::HashMap;
use ordered_float::OrderedFloat;

use lazy_static::lazy_static;

lazy_static! {
    // taken from http://www.fitaly.com/board/domper3/posts/136.html
    static ref FREQS: HashMap<char, f64> = {
        let counts = [
            (' ', 407934),
            ('!', 170),
            ('"', 5804),
            ('#', 425),
            ('$', 1333),
            ('%', 380),
            ('&', 536),
            ('\'', 5816),
            ('(', 5176),
            (')', 5307),
            ('*', 1493),
            ('+', 511),
            (',', 17546),
            ('-', 32638),
            ('.', 35940),
            ('/', 3681),
            ('0', 13109),
            ('1', 10916),
            ('2', 7894),
            ('3', 4389),
            ('4', 3204),
            ('5', 3951),
            ('6', 2739),
            ('7', 2448),
            ('8', 2505),
            ('9', 2433),
            (':', 10347),
            (';', 2884),
            ('<', 2911),
            ('=', 540),
            ('>', 2952),
            ('?', 3503),
            ('@', 173),
            ('[', 205),
            ('\\', 37),
            (']', 210),
            ('^', 8),
            ('_', 2755),
            ('`', 21),
            ('a', 123287   + 7444),
            ('b', 24227    + 5140),
            ('c', 50211    + 9283),
            ('d', 59577    + 7489),
            ('e', 203824   + 6351),
            ('f', 32616    + 3365),
            ('g', 37064    + 4459),
            ('h', 65217    + 5515),
            ('i', 116488   + 7631),
            ('j', 2061     + 4102),
            ('k', 16047    + 1633),
            ('l', 75450    + 4476),
            ('m', 39060    + 8386),
            ('n', 118108   + 4954),
            ('o', 137119   + 4378),
            ('p', 36791    + 6211),
            ('q', 1774     + 751),
            ('r', 101201   + 5986),
            ('s', 103814   + 9512),
            ('t', 151376   + 7895),
            ('u', 49901    + 1934),
            ('v', 20109    + 2119),
            ('w', 30974    + 6005),
            ('x', 4635     + 815),
            ('y', 26924    + 722),
            ('z', 1417     + 180),
            ('{', 62),
            ('|', 16),
            ('}', 61),
            ('~', 8),
        ];
        let total: usize = counts.iter().map(|(_,n)|n).sum();
        counts.iter().map(|(c,n)| (*c, *n as f64 / total as f64)).collect()
    };
}

fn calc_letter_freqs(s: &str) -> Option<HashMap<char, f64>> {
    let mut counts: HashMap<char, usize> = HashMap::new();

    let mut total = 0;
    for c in s.to_ascii_lowercase().chars() {
        if !FREQS.contains_key(&c) {
            continue;
        }
        total += 1;
        if let Some(n) = counts.get_mut(&c) {
            *n += 1;
        } else {
            counts.insert(c, 1);
        }
    }

    if total > 0 {
        Some(counts.into_iter().map(|(k,v)| (k, v as f64 / total as f64)).collect())
    } else {
        None
    }
}

fn mean_square(i: impl Iterator<Item = f64>) -> Option<f64> {
    let mut sum = 0.0;
    let mut count = 0;
    for n in i {
        count += 1;
        sum += n*n;
    }
    if count > 0 {
        Some(sum / (count as f64))
    } else {
        None
    }
}

fn score_text(s: &str) -> Option<f64> {
    calc_letter_freqs(s).and_then(|freqs| {
        mean_square(FREQS.iter().map(|(c, freq)| {
            if let Some(n) = freqs.get(&c) {
                (n-freq)*(n-freq)
            } else {
                freq*freq
            }
        }))
    })
}

fn byte_xor(s: &[u8], b: u8) -> Vec<u8> {
    s.iter().map(|c| c ^ b).collect()
}

pub fn byte_xor_solutions(input: &[u8]) -> Vec<(String, u8, f64)> {
    let mut result: Vec<(String, u8, f64)> = (std::u8::MIN..=std::u8::MAX).filter_map(|c| {
        let rotated = byte_xor(&input, c);
        String::from_utf8(rotated).ok().and_then(|s| {
            score_text(&s).map(|score| (s, c, score))
        })
    }).collect();

    result.sort_by_key(|e| OrderedFloat(e.2));

    result
}

pub fn best_score_byte_xor(input: &[u8]) -> (String, u8, f64) {
    return byte_xor_solutions(input).swap_remove(0);
    let mut key = 0;
    let mut best_score = std::f64::MAX;
    let mut result = String::new();

    for c in std::u8::MIN..=std::u8::MAX {
        let rotated = byte_xor(&input, c);
        if let Ok(s) = String::from_utf8(rotated) {
            if let Some(score) = score_text(&s) {
                if score < best_score {
                    best_score = score;
                    key = c;
                    result = s;
                }
            }
        }
    }

    (result, key, best_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c3() {
        let input = hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").unwrap();
        assert_eq!(best_score_byte_xor(&input).0, "Cooking MC's like a pound of bacon".to_owned());
    }

    #[test]
    fn test_tally() {
        let input = "aaaabbcd";
        let expected: HashMap<char, f64> = [
            ('a', 0.5),
            ('b', 0.25),
            ('c', 0.125),
            ('d', 0.125),
        ].iter().cloned().collect();

        assert_eq!(expected, calc_letter_freqs(&input).unwrap());
    }

    #[test]
    fn test_mean_square() {
        let input = vec![1.0, 2.0, 3.0];
        assert_eq!(mean_square(input.into_iter()).unwrap(), 14.0f64 / 3.0);
    }

    #[test]
    fn test_single_byte_xor() {
        let input = vec![0b11111111, 0b00000000, 0b10101010];
        let b = 0b11100111;
        assert_eq!(byte_xor(&input, b), vec![0b00011000, 0b11100111, 0b01001101]);
    }
}
