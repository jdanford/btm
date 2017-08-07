use std::str;

use ternary::tryte;
use ternary::text::*;

#[test]
fn text_encode_decode() {
    let mut trytes = [tryte::ZERO; 256];
    let s1 = "⸘I like to éat 🍎 and 🍌 wheñ it is 100℉ oütside‽";

    let len1 = encode_str(&mut trytes, s1).expect("encoding error");
    let (s2, len2) = decode_str(&trytes).expect("decoding error");

    assert_eq!(len1, len2);
    assert_eq!(s1, &s2[..]);
}
