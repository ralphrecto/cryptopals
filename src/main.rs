extern crate base64;

mod cryptopals {
    pub mod set1;
    pub mod hexutils;
    pub mod freq_analysis;
    pub mod fileutils;
    pub mod xorutils;
    pub mod strutils;
}

fn cryptomain() -> () {
//    cryptopals::set1::challenge1();
//    cryptopals::set1::challenge2();
//    cryptopals::set1::challenge3();
//    cryptopals::set1::challenge4();
//    cryptopals::set1::challenge5();
    cryptopals::set1::challenge6();

}

fn main() -> () {
    cryptomain();
}
