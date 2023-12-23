#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
extern crate alloc;

use alloc::string::{String, ToString};
use codec::{Encode, Decode};

#[derive(Debug, PartialEq, Encode, Decode)]
struct Test(u8);

#[no_mangle]
pub fn main() -> i32 {
    println!("Hello world from user mode program!");
    let a = Test(3);
    let a_encoded = a.using_encoded(|ref slice| {
        println!("{:?}", slice);
        String::from_utf8_lossy(slice).into_owned()
    });

    println!("{}", a_encoded);

    let b = Test::decode(&mut a_encoded.as_bytes()).expect("Decode failed");
    println!("{:?}", b);
    0
}
