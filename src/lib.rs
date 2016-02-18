extern crate rustc_serialize;

mod hex_to_base64;
mod xor_from_hex;

#[cfg(test)]
mod test {
    #[test]
    fn hex_to_base64() {
        use hex_to_base64::hex_to_base64;
        use std::str::FromStr;
        let from = String::from_str("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap();
        let to = String::from_str("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t").unwrap();
        assert_eq!(hex_to_base64(&from), to)
    }

    #[test]
    fn xor_from_hex() {
        use xor_from_hex::xor_from_hex;
        let s1 = "1c0111001f010100061a024b53535009181c".to_owned();
        let s2 = "686974207468652062756c6c277320657965".to_owned();
        let expected = "746865206b696420646f6e277420706c6179";
        assert_eq!(xor_from_hex(&s1, &s2), expected)
    }
}
