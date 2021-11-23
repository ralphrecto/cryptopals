use cryptopals::hexutils;

/// Consider xor as a byte-wise operation. This function takes the given 
/// and returns all possible byte keys and the bytestring (as a string)
/// resulting from xor-ing all of the given string's bytes with the byte key.
pub fn bytexor_keygen(v: &Vec<u8>) -> Vec<(String, u8)> {
    (0..256).map(|b| b as u8)
        .map(|b: u8| {
            let word: String = v.iter()
                .map(|&x: &u8| (x ^ b) as char)
                .collect();
            (word, b)
        }).collect()
}

/// Apply a rolling xor of v using key. The returned vector will be the same
/// length as v.
pub fn rolling_xor(v: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
    v.iter()
        .enumerate()
        .map(|(i, byte)| {
            *byte ^ key[i % key.len()]
        }).collect()
}
