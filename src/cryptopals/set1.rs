use cryptopals::utils;
use base64;
use cryptopals::freq_analysis;

fn challenege_start(set: u8, num: u8) -> () {
    println!("\n\n");
    println!("Set {}, challenge {} start ===========================================", set, num);
}

fn challenege_end(set: u8, num: u8) -> () {
    println!("Set {}, challenge {} end =============================================", set, num);
    println!("\n\n");
}

pub fn challenge1() -> () {
    challenege_start(1, 1);
    let s = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    println!("{}", base64::encode(&utils::hexstr_to_bytevec(s)));
    challenege_end(1, 1);
}

pub fn challenge2() -> () {
    challenege_start(1, 2);
    let s1 = "1c0111001f010100061a024b53535009181c";
    let s2 = "686974207468652062756c6c277320657965";

    let v1: Vec<u8> = utils::hexstr_to_bytevec(s1);
    let v2: Vec<u8> = utils::hexstr_to_bytevec(s2);

    let xor: Vec<u8> = v1.iter().zip(v2.iter())
        .map(|(b1, b2)| b1 ^ b2)
        .collect();

    let fin: String = utils::bytevec_to_hexstr(&xor);
    println!("{}", fin);
    challenege_end(1, 2);
}

pub fn challenge3() -> () {
    challenege_start(1, 3);
    let s = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let v: Vec<u8> = utils::hexstr_to_bytevec(s);

    // Consider xor as a byte-wise operation, so that the key is a single byte.
    // Return all keys + their corresponding word where each char in the word is
    // an ASCII printable char.
    let printable_word_and_keys: Vec<(Vec<u8>, u8)> = (0..255).map(|b: u8| {
        let word: Vec<u8> = v.iter().map(|x| x ^ b).collect();
        (word, b)
    }).collect();

    for (raw_word, key) in printable_word_and_keys {
        let word: String = raw_word.iter()
            .map(|&x: &u8| x as char)
            .collect();
        match freq_analysis::xi_square(&word) {
            Some(score) => {
                println!("{}, key: {:#x}, score: {}", word, key, score)
            },
            None => { }
        }
    }
    challenege_end(1, 3);
}
