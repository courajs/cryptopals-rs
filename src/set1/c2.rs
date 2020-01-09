fn fixed_xor (a: &[u8], b: &[u8]) -> Option<Vec<u8>> {
    if a.len() != b.len() {
        return None;
    }
    let mut result = Vec::with_capacity(a.len());
    for i in 0..a.len() {
        result.push(a[i] ^ b[i]);
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c2() {
        let a = hex::decode("1c0111001f010100061a024b53535009181c").unwrap();
        let b = hex::decode("686974207468652062756c6c277320657965").unwrap();

        let expected = hex::decode("746865206b696420646f6e277420706c6179").unwrap();

        assert_eq!(fixed_xor(&a, &b).unwrap(), expected);
    }
}
