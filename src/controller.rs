use crate::utils::*;
use mainline::{Dht, MutableItem, SigningKey};
use std::collections::HashMap;

pub async fn set_proxies(announcer_key: SigningKey, new_proxies: HashMap<String, Vec<String>>) {
    let client = Dht::builder().build().unwrap();
    client
        .put_mutable(MutableItem::new(
            announcer_key,
            serde_json::to_string(&new_proxies).unwrap().into(),
            gen_seq(),
            None,
        ))
        .unwrap();
}

pub async fn set_checkers(announcer_key: SigningKey, new_checkers: Vec<String>) {
    let client = Dht::builder().build().unwrap();
    client
        .put_mutable(MutableItem::new(
            announcer_key,
            serde_json::to_string(&new_checkers).unwrap().into(),
            gen_seq(),
            None,
        ))
        .unwrap();
}
