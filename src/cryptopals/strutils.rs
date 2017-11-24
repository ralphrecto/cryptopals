use cryptopals::hexutils;

/// Finds the hamming distance between two bitstrings. Since the distance
/// is only defined for strings of the same length, this will panic if the
/// vectors are of different lengths.
pub fn hamming_dist(v1: &Vec<u8>, v2: &Vec<u8>) -> u32 {
    if v1.len() != v2.len() {
        panic!("Trying to get hamming distance of different length strings.");
    }

    v1.iter().zip(v2.iter())
        .map(|(b1, b2)| {
            hexutils::count_set_bits(b1 ^ b2) as u32
        }).sum()
}
