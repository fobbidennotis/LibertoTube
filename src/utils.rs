use mainline::*;
use std::time::SystemTime;

pub fn gen_seq() -> i64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_micros() as i64
}

pub fn get_message(client: &Dht, verify_key: &[u8; 32]) -> Option<MutableItem> {
    Some(client.get_mutable(verify_key, None, None).ok()?.last()?)
}

pub fn from_hex(s: &str) -> Vec<u8> {
    if s.len() % 2 != 0 {
        panic!("Number of Hex characters should be even");
    }

    let mut bytes = Vec::with_capacity(s.len() / 2);

    for i in 0..s.len() / 2 {
        let byte_str = &s[i * 2..(i * 2) + 2];
        let byte = u8::from_str_radix(byte_str, 16).expect("Invalid hex character");
        bytes.push(byte);
    }

    bytes
}

pub fn to_hex(bytes: &[u8]) -> String {
    let hex_chars: String = bytes.iter().map(|byte| format!("{:02x}", byte)).collect();

    hex_chars
}
