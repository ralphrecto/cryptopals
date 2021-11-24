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
    const MIN_KEYSIZE: usize = 2;
    const MAX_KEYSIZE: usize = 40;
    const NUM_TOP_KEYSIZES: usize = 1;
    let ciphertext= fileutils::read_file("data/set1-challenge6.txt")?;
    let raw_bytes = base64::decode(&ciphertext.replace('\n', "")).unwrap();

    // Part 1: find the most probable key size/s.
    let mut keysize_scores: Vec<(i32, f64)> = Vec::with_capacity(MAX_KEYSIZE);
    for keysize in MIN_KEYSIZE..=MAX_KEYSIZE {
        let mut i = 0;
        let mut hamming_dist_sum = 0.0;
        let mut num_pairs = 0;

        // NOTE: this loop implements step 3 of the problem.
        // The text of the step says to only do the first and second keysize worth
        // of bytes, but this is actually not sufficient; the keysizes found using
        // that method produce garbage. Here we take the normalized hamming distances
        // for all consecutive pairs of blocks for the WHOLE input.
        while i + (keysize*2) < raw_bytes.len() {
            let block1 = &raw_bytes[i..i+keysize];
            let block2 = &raw_bytes[i+keysize..i+(keysize*2)];

            let d = hamming_dist(&block1.to_vec(), &block2.to_vec());
            let normalized_d = d as f64 / keysize as f64;

            hamming_dist_sum += normalized_d;
            num_pairs += 1;
            i += keysize*2;
        }

        let keysize_score = hamming_dist_sum / num_pairs as f64;
        keysize_scores.push((keysize as i32, keysize_score));
    }
    keysize_scores.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let top_keysizes = &keysize_scores[0..NUM_TOP_KEYSIZES];

    // Part 2: generate transposed blocks.
    // keysize -> block num -> raw bytes
    let mut transposed_blocks: HashMap<i32, HashMap<i32, Vec<u8>>> = HashMap::new();
    for (probable_keysize, _) in top_keysizes {
        let keysize_map = transposed_blocks
            .entry(*probable_keysize)
            .or_insert_with(HashMap::new);

        for i in 0..raw_bytes.len() {
            let block_i = i as i32 % *probable_keysize;
            let block = keysize_map
                .entry(block_i)
                .or_insert_with(|| vec![]);

            block.push(raw_bytes[i]);
        }
    }

    // Part 3: generate most likely byte key for each transposed block.
    let mut cand_keys: HashMap<i32, Vec<u8>> = HashMap::new();
    for (keysize, keysize_map) in transposed_blocks {
        let mut key: Vec<u8> = vec![0; keysize as usize];

        for (block_i, block) in keysize_map {
            let keygen_pairs = xorutils::bytexor_keygen(&block);

            let mut min_keybyte: u8 = 0;
            let mut min_score: f64 = f64::MAX;

            for (plaintext, keybyte) in keygen_pairs {
                let score = match freq_analysis::chi_square(&plaintext) {
                    Some(chi_sq) => chi_sq,
                    None => f64::MAX
                };

                if score < min_score {
                    min_score = score;
                    min_keybyte = keybyte;
                }
            }

            key[block_i as usize] = min_keybyte;
        }

        cand_keys.insert(keysize, key);
    }

    for (_, key) in cand_keys {
        let unencrypted: String = xorutils::rolling_xor(&raw_bytes, &key)
            .iter()
            .map(|byte| *byte as char)
            .collect();

        println!("key {:?}, unencrypted: {}", key, unencrypted);
    }

    Ok(())
}
