#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
extern crate alloc;

#[no_mangle]
pub fn main() -> i32 {
    println!("Hello world from user mode program!");
    tests::test_parity_scale_codec();
    0
}

mod tests {
    // 公用的use
    use super::*;
    use alloc::string::String;

    // parity-scale-codec的use
    use codec::{Decode, Encode};
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Test(String);

    pub fn test_parity_scale_codec() {
        let a = Test(String::from("ookami-mio"));
        let a_encoded = a.using_encoded(|ref slice| String::from_utf8_lossy(slice).into_owned());
        let a_decoded = Test::decode(&mut a_encoded.as_bytes()).expect("Decode failed");

        println!(
            "{:?} is encoded as {:?}, And then decoded back as {:?}.",
            a, a_encoded, a_decoded
        );
    }
}
