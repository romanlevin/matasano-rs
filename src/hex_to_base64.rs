use rustc_serialize::hex::FromHex;
use rustc_serialize::base64::{ToBase64, Config, CharacterSet, Newline};

pub fn default_config() -> Config {
    Config{ char_set: CharacterSet::Standard, newline: Newline::LF, pad: false, line_length: None }
}

pub fn hex_to_base64(hex: &String) -> String  {
    hex.from_hex().unwrap().to_base64(default_config())
}
