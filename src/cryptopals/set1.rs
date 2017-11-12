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

    let fin: String = utils::bytevec_to_hexstr(xor);
    println!("{}", fin);
}