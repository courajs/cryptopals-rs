use crate::set1::c3;

fn best_score_of_all(inputs: impl Iterator<Item = Vec<u8>>) -> (String, u8, f64) {
    let mut result = (String::new(), 0, std::f64::MAX);

    for l in inputs {
        // println!("hi {}", l);
        let mut these = c3::byte_xor_solutions(&l);
        if these.len() > 0 {
            let t = these.swap_remove(0);
            if t.2 < result.2 {
                result = t;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::BufReader;
    use std::io::BufRead;

    #[test]
    fn c4() {

        let file = File::open("/Users/aaron/dev/cryptopals-rs/src/set1/c4.txt").unwrap();
        let reader = BufReader::new(file);
        let inputs = reader.lines().filter_map(Result::ok).map(hex::decode).filter_map(Result::ok);
        let r = best_score_of_all(inputs);
        assert_eq!((r.0,r.1), ("Now that the party is jumping\n".to_owned(), 53));
    }
}
