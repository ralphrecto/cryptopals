extern crate base64;

mod cryptopals {
    pub mod set1;
    pub mod utils;
}

fn cryptomain() -> () {
    cryptopals::set1::challenge1();
    cryptopals::set1::challenge2();
}

fn main() -> () {
    cryptomain();
}
