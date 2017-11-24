use base64;
use cryptopals::hexutils;
use cryptopals::freq_analysis;
use cryptopals::fileutils;
use cryptopals::xorutils;
use cryptopals::strutils;

pub fn challenge1() -> () {
    let s = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    println!("{}", base64::encode(&hexutils::hexstr_to_bytevec(s)));
}

pub fn challenge2() -> () {
    let s1 = "1c0111001f010100061a024b53535009181c";
    let s2 = "686974207468652062756c6c277320657965";

    let v1: Vec<u8> = hexutils::hexstr_to_bytevec(s1);
    let v2: Vec<u8> = hexutils::hexstr_to_bytevec(s2);

    let xor: Vec<u8> = v1.iter().zip(v2.iter())
        .map(|(b1, b2)| b1 ^ b2)
        .collect();

    let fin: String = hexutils::bytevec_to_hexstr(&xor);
    println!("{}", fin);
}

fn chi_square_printer(word: &str, keyword_and_keys: &Vec<(String, u8)>) -> () {
    for &(ref keyword, key) in keyword_and_keys {
        match freq_analysis::chi_square(keyword) {
            Some(score) => {
                println!("word: {}, keyword: {}, key: {:#x}, score: {}", word, keyword, key, score)
            },
            None => { }
        }
    }
}

pub fn challenge3() -> () {
    let s = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let all_words_and_keys: Vec<(String, u8)> = xorutils::bytexor_keygen(s);
    chi_square_printer(s, &all_words_and_keys);
}

pub fn challenge4() -> () {
    let lines: Vec<String> =
        fileutils::read_lines("./data/set1-challenge4.txt").unwrap();

    for word in lines {
        let all_words_and_keys: Vec<(String, u8)> = xorutils::bytexor_keygen(&word);
        chi_square_printer(&word, &all_words_and_keys);
    }
}

pub fn challenge5() -> () {
    let str = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let str_bytes: Vec<u8> = hexutils::str_to_bytevec(str);
    let key_bytes: Vec<u8> = hexutils::str_to_bytevec("ICE");
    let result: Vec<u8> = xorutils::rolling_xor(&str_bytes, &key_bytes);

    println!("{}", hexutils::bytevec_to_hexstr(&result));
}

pub fn challenge6() -> () {
    let v1: Vec<u8> = hexutils::str_to_bytevec("this is a test");
    let v2: Vec<u8> = hexutils::str_to_bytevec("wokka wokka!!!");
}
