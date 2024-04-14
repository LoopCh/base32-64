
mod base64;
mod base32;
mod utils;

use base64::Base64;
use base32::Base32;

fn main() {
    let encode_base64 = Base64::encode(&"Hello".to_string());
    let encode_base32 = Base32::encode(&"Привет".to_string());
    let _decode_base64 = Base64::decode(&encode_base64);
    let decode_base32 = Base32::decode(&encode_base32);
    dbg!(encode_base32, decode_base32);
}
