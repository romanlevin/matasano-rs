use rustc_serialize::hex::{FromHex, ToHex};

pub fn xor_from_hex(hex1: &String, hex2: &String) -> String{
    let bytes1 = hex1.from_hex().unwrap();
    let bytes2 = hex2.from_hex().unwrap();
    let pairs = bytes1.iter().zip(bytes2.iter());
    let xored: Vec<u8> = pairs.map(|(b1, b2)| b1 ^ b2).collect();
    xored.to_hex()
}
