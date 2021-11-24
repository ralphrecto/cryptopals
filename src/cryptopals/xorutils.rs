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

#[test]
fn test_bytexor_keygen_length() {
    let plaintexts = bytexor_keygen(&vec![0, 0, 0]);
}

#[test]
fn test_bytexor_keygen_identity() {
    let plaintexts = bytexor_keygen(&vec![0, 0, 0]);

    for (plaintext, key) in plaintexts {
        let s: String = vec![key, key, key]
            .iter()
            .map(|b| *b as char)
            .collect();

        assert_eq!(plaintext, s);
    }
}

#[test]
fn test_rolling_xor_ids() {
    let src = vec![0, 0, 0, 0];
    let ones = rolling_xor(&src, &vec![0xff]);

    assert_eq!(&vec![0xff, 0xff, 0xff, 0xff], &ones);

    let zero_ones = rolling_xor(&src, &vec![0x00, 0xff]);
    assert_eq!(&vec![0x00, 0xff, 0x00, 0xff], &zero_ones);
}

#[test]
fn test_rolling_xor_ice() {
    let key = "ICE".as_bytes().to_vec();
    let plaintext_bytes = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal".as_bytes().to_vec();

    let ciphertext = rolling_xor(&plaintext_bytes, &key);

    assert_eq!(
        hexutils::hexstr_to_bytevec("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"),
        ciphertext
    );
}