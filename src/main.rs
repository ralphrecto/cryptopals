extern crate base64;
extern crate aes;

mod cryptopals {
    pub mod set1;
    pub mod hexutils;
    pub mod freq_analysis;
    pub mod fileutils;
    pub mod xorutils;
    pub mod strutils;
    pub mod blockutils;
}

fn cryptomain() -> () {
    // cryptopals::set1::challenge1();
    // cryptopals::set1::challenge2();
    // cryptopals::set1::challenge3();
    // cryptopals::set1::challenge4();
    // cryptopals::set1::challenge5();
    // cryptopals::set1::challenge6();
    cryptopals::set1::challenge7();
}

fn main() -> () {
    cryptomain();
}
