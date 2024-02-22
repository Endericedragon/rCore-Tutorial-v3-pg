#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
extern crate alloc;

#[no_mangle]
pub fn main() -> i32 {
    println!("Hello world from user mode program!");
    tests::test_parity_scale_codec();
    tests::test_adler();
    0
}

mod tests {
    // 公用的use
    use super::*;
    use alloc::string::{String, ToString};
    use core::hash::Hasher;

    // parity-scale-codec的use
    use codec::{Decode, Encode};
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Test(String);

    pub fn test_parity_scale_codec() {
        let a = Test(String::from("ookami-mio"));
        let a_encoded = a.using_encoded(|ref slice| String::from_utf8_lossy(slice).into_owned());
        let b = Test::decode(&mut a_encoded.as_bytes()).expect("Decode failed");

        println!(
            "{:?} is encoded as {:?}, And then decoded back as {:?}.",
            a, a_encoded, b
        );
    }

    // adler的use
    use adler::Adler32;

    pub fn test_adler() {
        let mut adler = Adler32::new();
        let a = Test(String::from("Adler-32 test."));
        let a_encoded = a.using_encoded(|ref slice| String::from_utf8_lossy(slice).into_owned());
        adler.write_slice(a_encoded.as_bytes());
        println!("Adler32 checksum of {:?} is {:?}", a, adler.checksum());
    }
}
