use base64;
use cryptopals::hexutils;
use cryptopals::freq_analysis;
use cryptopals::fileutils;

fn challenege_start(set: u8, num: u8) -> () {
    println!("Set {}, challenge {} start ===========================================", set, num);
}

fn challenege_end(set: u8, num: u8) -> () {
    println!("Set {}, challenge {} end =============================================", set, num);
}

pub fn challenge1() -> () {
    challenege_start(1, 1);
    let s = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    println!("{}", base64::encode(&hexutils::hexstr_to_bytevec(s)));
    challenege_end(1, 1);
}

pub fn challenge2() -> () {
    challenege_start(1, 2);
    let s1 = "1c0111001f010100061a024b53535009181c";
    let s2 = "686974207468652062756c6c277320657965";

    let v1: Vec<u8> = hexutils::hexstr_to_bytevec(s1);
    let v2: Vec<u8> = hexutils::hexstr_to_bytevec(s2);

    let xor: Vec<u8> = v1.iter().zip(v2.iter())
        .map(|(b1, b2)| b1 ^ b2)
        .collect();

    let fin: String = hexutils::bytevec_to_hexstr(&xor);
    println!("{}", fin);
    challenege_end(1, 2);
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
    challenege_start(1, 3);
    let s = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let all_words_and_keys: Vec<(String, u8)> = hexutils::bytexor_keygen(s);
    chi_square_printer(s, &all_words_and_keys);
    challenege_end(1, 3);
}

pub fn challenge4() -> () {
    challenege_start(1, 4);
    let lines: Vec<String> =
        fileutils::read_lines("./data/set1-challenge4.txt").unwrap();

    for word in lines {
        let all_words_and_keys: Vec<(String, u8)> = hexutils::bytexor_keygen(&word);
        chi_square_printer(&word, &all_words_and_keys);
    }

    challenege_end(1, 4);
}
