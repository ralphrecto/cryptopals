extern crate base64;

/// Convert hex character to a nibble, returned in the lower half of a byte.
pub fn hex_to_nibble(s: char) -> u8 {
    match s.to_digit(16) {
        Some(int) => int as u8,
        None => panic!("Given char was not a hex char."),
    }
}

/// Convert a nibble to a hex char. Panics if the given byte does not contain
/// a nibble (i.e. outside of decimal range [0, 15]).
pub fn nibble_to_hex(b: u8) -> char {
    match b {
        // U+0030 = '0'
        0...9 => (0x30 + b) as char,
        10 => 'a',
        11 => 'b',
        12 => 'c',
        13 => 'd',
        14 => 'e',
        15 => 'f',
        _ => panic!("Given byte does not contain a nibble."),
    }
}

/// Convert a hex string to a vector of bytes.
pub fn hexstr_to_bytevec(s: &str) -> Vec<u8> {
    // Directly convert each hex char to a nibble.
    let raw_nibbles: Vec<u8> = s.chars()
        .map(hex_to_nibble)
        .collect();

    // Since the nibbles are in the lower half of bytes, form a full byte
    // from pairs of nibbles by left shifting upper one by half a byte
    // and adding the pair together.
    return raw_nibbles
        .chunks(2)
        .map(|chunk| (chunk[0] << 4) | chunk[1])
        .collect();
}

/// Convert a vector of bytes into a string of hex chars.
pub fn bytevec_to_hexstr(v: &Vec<u8>) -> String {
    let mut acc_string: String = String::from("");
    for byte in v {
        acc_string.push(nibble_to_hex(byte >> 4));
        acc_string.push(nibble_to_hex(byte & 0x0f));
    }
    return acc_string;
}

/// Consider xor as a byte-wise operation. This function takes the given string
/// and returns all possible byte keys and the bytestring (as a string)
/// resulting from xor-ing all of the given string's bytes with the byte key.
pub fn bytexor_keygen(s: &str) -> Vec<(String, u8)> {
    let s_bytevec: Vec<u8> = hexstr_to_bytevec(s);
    (0..256).map(|b| b as u8)
        .map(|b: u8| {
            let word: String = s_bytevec.iter()
                .map(|&x: &u8| (x ^ b) as char)
                .collect();
            (word, b)
        }).collect()
}