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
pub fn byte_to_nibble(b: u8) -> char {
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
        .map(|chunk| (chunk[0] << 4) + chunk[1])
        .collect();
}

/// Convert a vector of bytes into a string.
pub fn bytevec_to_hexstr(v: Vec<u8>) -> String {
    return v.into_iter()
        .flat_map(|b: u8| {
            let upper_nibble = b >> 4;
            let lower_nibble = b & 0x0f;
            vec![byte_to_nibble(upper_nibble), byte_to_nibble(lower_nibble)]
        }).collect();
}