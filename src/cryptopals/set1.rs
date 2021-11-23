use std::{collections::HashMap, io};
use base64;
use cryptopals::{
    hexutils,
    freq_analysis,
    fileutils,
    xorutils,
    strutils
};

use super::{hexutils::hex_to_nibble, strutils::hamming_dist};

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
    let v = hexutils::hexstr_to_bytevec(s);
    let all_words_and_keys: Vec<(String, u8)> = xorutils::bytexor_keygen(&v);
    chi_square_printer(s, &all_words_and_keys);
}

pub fn challenge4() -> () {
    let lines: Vec<String> =
        fileutils::read_lines("./data/set1-challenge4.txt").unwrap();

    for word in lines {
        let v = hexutils::hexstr_to_bytevec(&word);
        let all_words_and_keys: Vec<(String, u8)> = xorutils::bytexor_keygen(&v);
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

pub fn challenge6() -> io::Result<()> {
    let ciphertext= fileutils::read_file("data/set1-challenge6.txt")?;
    let decoded = base64::decode(&ciphertext.replace('\n', "")).unwrap();

    let mut probable_keysize: i32 = -1;
    let mut min_normalized_d = f64::MAX;
    for keysize in 2..=40 {
        let block1 = &decoded[0..keysize];
        let block2 = &decoded[keysize..keysize*2];

        let block3 = &decoded[keysize*2..keysize*3];
        let block4 = &decoded[keysize*3..keysize*4];

        let d1 = hamming_dist(&block1.to_vec(), &block2.to_vec());
        let d2 = hamming_dist(&block3.to_vec(), &block4.to_vec());

        let normalized_d1 = d1 as f64 / keysize as f64;
        let normalized_d2 = d2 as f64 / keysize as f64;

        let avg_normalized_d = (normalized_d1 + normalized_d2) / 2.0;

        println!("avg d {}, keysize {}", avg_normalized_d, keysize);
        println!("d1 {}, keysize {}", normalized_d1, keysize);

        if normalized_d1 < min_normalized_d {
            probable_keysize = keysize as i32;
            min_normalized_d = normalized_d1;
        }
    }

    let mut transposed_blocks: HashMap<i32, Vec<u8>> = HashMap::new();
    for i in 0..decoded.len() {
        let block_i = i as i32 % probable_keysize;
        if !transposed_blocks.contains_key(&block_i) {
            transposed_blocks.insert(block_i, vec![]);
        }

        let block = transposed_blocks.get_mut(&block_i).unwrap();
        block.push(decoded[i]);
    }

    let key: Vec<u8> = Vec::with_capacity(probable_keysize as usize);
    for (block_i, block) in transposed_blocks {
        let keygen_pairs = xorutils::bytexor_keygen(&block);
        for (plaintext, keybyte) in keygen_pairs {
            // println!("plaintext: {}", &plaintext);
            match freq_analysis::chi_square(&plaintext) {
                Some(chi_sq) => println!("keybyte {}, chi sq {}", keybyte, chi_sq),
                None => {
                    // println!("None!") // The decoded string isn't even english, so this is not the key
                }
            }
        }
    }

    Ok(())
}
