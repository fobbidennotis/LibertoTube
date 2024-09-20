use mainline::*;
use std::net::{SocketAddr, ToSocketAddrs};

type AnyError = Box<dyn std::error::Error>;

pub async fn check_availability(hostname: &str) -> Result<bool, AnyError> {
    Ok(reqwest::get(hostname).await?.status().is_success())
}

pub async fn server() {
    let server = mainline::dht::Dht::builder()
        .server()
        .port(8888)
        .build()
        .unwrap();

    dbg!(server.local_addr());
    let key = SigningKey::from_bytes(&[7u8; 32]);
    println!("{}", to_hex(key.verifying_key().as_bytes().into()));

    let mut last_seq = 1;
    let mut stdin_lines = std::io::stdin().lines();

    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    while let Some(Ok(line)) = stdin_lines.next() {
        send_msg(&server, key.clone(), line.into(), last_seq).unwrap();
        last_seq += 1;
    }

    tokio::time::sleep(std::time::Duration::from_secs(100)).await;
}

fn send_msg(server: &Dht, key: SigningKey, content: Vec<u8>, seq: i64) -> Result<(), AnyError> {
    dbg!(server.put_mutable(MutableItem::new(key, content.into(), seq, None))?);
    Ok(())
}

pub async fn client() {
    let client = mainline::dht::Dht::builder()
        .bootstrap(&[String::from("127.0.0.1:8888")])
        .build()
        .unwrap();
    let key = SigningKey::from_bytes(&[7u8; 32]);
    let sender_pubkey = key.verifying_key();
    let sender_pubkey = sender_pubkey.as_bytes();

    let mut last_seq = 1;
    loop {
        dbg!(last_seq);
        let Some(msg) = get_message(&client, sender_pubkey, last_seq) else {
            continue;
        };
        println!("{}", std::str::from_utf8(msg.value()).unwrap());
        // tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        last_seq += 1;
    }
}

fn get_message(client: &Dht, verify_key: &[u8; 32], seq: i64) -> Option<MutableItem> {
    Some(
        client
            .get_mutable(verify_key, None, Some(seq))
            .ok()?
            .last()?,
    )
}

//
//

fn from_hex(s: String) -> SigningKey {
    if s.len() % 2 != 0 {
        panic!("Number of Hex characters should be even");
    }

    let mut bytes = Vec::with_capacity(s.len() / 2);

    for i in 0..s.len() / 2 {
        let byte_str = &s[i * 2..(i * 2) + 2];
        let byte = u8::from_str_radix(byte_str, 16).expect("Invalid hex character");
        bytes.push(byte);
    }

    SigningKey::try_from(bytes.as_slice()).expect("Invalid signing key")
}

fn to_hex(bytes: Vec<u8>) -> String {
    let hex_chars: String = bytes.iter().map(|byte| format!("{:02x}", byte)).collect();

    hex_chars
}
