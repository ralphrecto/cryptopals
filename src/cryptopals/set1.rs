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
    const NUM_TOP_KEYSIZES: usize = 5;
    let ciphertext= fileutils::read_file("data/set1-challenge6.txt")?;
    let decoded = base64::decode(&ciphertext.replace('\n', "")).unwrap();

    let mut keysize_scores: Vec<(i32, f64)> = Vec::with_capacity(40);
    for keysize in 2..=40 {
        let block1 = &decoded[0..keysize];
        let block2 = &decoded[keysize..keysize*2];

        let d = hamming_dist(&block1.to_vec(), &block2.to_vec());

        let normalized_d = d as f64 / keysize as f64;
        keysize_scores.push((keysize as i32, normalized_d));
    }

    keysize_scores.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let top_keysizes = &keysize_scores[0..NUM_TOP_KEYSIZES];

    // keysize -> block num -> decoded bytes
    let mut transposed_blocks: HashMap<i32, HashMap<i32, Vec<u8>>> = HashMap::new();
    for (probable_keysize, _) in top_keysizes {
        transposed_blocks.insert(*probable_keysize, HashMap::new());

        let keysize_map = transposed_blocks.get_mut(probable_keysize).unwrap();
        for i in 0..decoded.len() {
            let block_i = i as i32 % *probable_keysize;
            if !keysize_map.contains_key(&block_i) {
                keysize_map.insert(block_i, vec![]);
            }

            let block = keysize_map.get_mut(&block_i).unwrap();
            block.push(decoded[i]);
        }
    }

    for (keysize, keysize_map) in transposed_blocks {
        for (block_i, block) in keysize_map {
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
    }

    Ok(())
}
