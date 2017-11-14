use cryptopals::utils;
use base64;

pub fn challenge1() -> () {
    let s = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    println!("{}", base64::encode(&utils::hexstr_to_bytevec(s)));
}

pub fn challenge2() -> () {
    let s1 = "1c0111001f010100061a024b53535009181c";
    let s2 = "686974207468652062756c6c277320657965";

    let v1: Vec<u8> = utils::hexstr_to_bytevec(s1);
    let v2: Vec<u8> = utils::hexstr_to_bytevec(s2);

    let xor: Vec<u8> = v1.iter().zip(v2.iter())
        .map(|(b1, b2)| b1 ^ b2)
        .collect();

    let fin: String = utils::bytevec_to_hexstr(&xor);
    println!("{}", fin);
}

pub fn challenge3() -> () {
    let s = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let v: Vec<u8> = utils::hexstr_to_bytevec(s);

    // Consider xor as a byte-wise operation, so that the key is a single byte.
    // Return all keys + their corresponding word where each char in the word is
    // an ASCII printable char.
    let printable_word_and_keys: Vec<(Vec<u8>, u8)> = (0..255).map(|b: u8| {
        let word: Vec<u8> = v.iter().map(|x| x ^ b).collect();
        (word, b)
    }).filter(|&(ref word, key)| {
        word.iter().all(|&x| 0x20u8 <= x && x <= 0x7eu8)
    }).collect();

    for (raw_word, score) in printable_word_and_keys {
        let word: String = raw_word.iter()
            .map(|&x: &u8| x as char)
            .collect();

        println!("{}, key: {:#x}", word, score);
    }

    // TODO: form model using frequency analysis to find best, rather than eyeballing.
}
