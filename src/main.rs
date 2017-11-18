extern crate base64;

mod cryptopals {
    pub mod set1;
    pub mod utils;
    pub mod freq_analysis;
}

fn cryptomain() -> () {
    cryptopals::set1::challenge1();
    cryptopals::set1::challenge2();
    cryptopals::set1::challenge3();
}

fn main() -> () {
    cryptomain();
}
