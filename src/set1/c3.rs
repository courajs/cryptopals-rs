use std::collections::HashMap;

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
    let mut counts: HashMap<char, usize> = FREQS.iter().map(|(c,_)| (*c, 0)).collect();

    let mut total = 0;
    for c in s.to_ascii_lowercase().chars() {
        if counts.contains_key(&c) {
            total += 1;
            let n = counts.get_mut(&c).unwrap();
            *n += 1;
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
            let n = freqs.get(c).unwrap() - freq;
            n*n
        }))
    })
}

fn byte_xor(s: &[u8], b: u8) -> Vec<u8> {
    s.iter().map(|c| c ^ b).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c3() {
        let input = hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").unwrap();

        let mut results = Vec::new();

        for c in std::u8::MIN..=std::u8::MAX {
            let rotated = byte_xor(&input, c);
            if let Ok(s) = String::from_utf8(rotated) {
                if let Some(score) = score_text(&s) {
                    dbg!(c, &s);
                    results.push((score, s));
                }
            }
        }

        // use std::cmp::partial_cmp;
        results.sort_by(|a,b| {
            // println!("{}, {}", a.0, b.0);
            a.0.partial_cmp(&b.0).unwrap()
        });

        for i in 0..10 {
            println!("{}: {}", results[i].0, results[i].1);
        }

        todo!()
    }

    #[test]
    fn test_tally() {
        let input = "aaaabbcd";
        let expected: HashMap<char, f64> = [
            ('A', 0.5),
            ('B', 0.25),
            ('C', 0.125),
            ('D', 0.125),
            ('E', 0.0), ('T', 0.0), ('O', 0.0),
            ('I', 0.0), ('N', 0.0), ('S', 0.0),
            ('R', 0.0), ('H', 0.0), ('L', 0.0),
            ('U', 0.0), ('M', 0.0), ('F', 0.0),
            ('P', 0.0), ('G', 0.0), ('W', 0.0),
            ('Y', 0.0), ('V', 0.0), ('K', 0.0),
            ('X', 0.0), ('J', 0.0), ('Q', 0.0),
            ('Z', 0.0),
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
