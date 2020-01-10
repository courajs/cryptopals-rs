
fn repeating_key_xor(bytes: &[u8], key: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(bytes.len());
    for (a, b) in key.iter().cycle().zip(bytes.iter()) {
        result.push(a ^ b);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c5() {
        let input = b"Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";
        let key = b"ICE";

        let expected = hex::decode("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f").unwrap();

        assert_eq!(repeating_key_xor(input, key), expected)
    }
}

