use crate::utils::*;
use ed25519_dalek::VerifyingKey;
use mainline::*;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

pub async fn check_availability(hostname: &str) -> usize {
    let Ok(resp) = reqwest::get(hostname).await else {
        return 0;
    };
    if resp.status().is_success() {
        10
    } else {
        0
    }
}

// let announcer_pubkey = VerifyingKey::try_from(
// from_hex("ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c").as_slice(),
// )
// .unwrap();
// let checker_key = from_hex("ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c");

pub async fn checker_server(announcer_pubkey: VerifyingKey, checker_key: SigningKey) {
    let proxies_to_check_: Arc<RwLock<HashMap<String, Vec<String>>>> =
        Arc::new(RwLock::new(HashMap::new()));

    let proxies_to_check = proxies_to_check_.clone();
    let proxies_fetcher_worker = tokio::spawn(async move {
        let client = mainline::dht::Dht::builder().build().unwrap();
        let mut last_seq = 0;
        loop {
            let Some(msg) = get_message(&client, announcer_pubkey.as_bytes()) else {
                continue;
            };

            if last_seq != *msg.seq() {
                let value = std::str::from_utf8(msg.value()).unwrap();
                last_seq = *msg.seq();
                *proxies_to_check.write().await = serde_json::from_str(value).unwrap();
            }

            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    });

    let proxies_to_check = proxies_to_check_.clone();
    let proxies_checker_worker = tokio::spawn(async move {
        let client = mainline::dht::Dht::builder().build().unwrap();
        loop {
            let mut results: HashMap<String, HashMap<String, usize>> = HashMap::new();
            let proxies_to_check_ = proxies_to_check.read().await;
            for (site, proxies) in proxies_to_check_.iter() {
                let mut results_inner = HashMap::new();
                for proxy in proxies.iter() {
                    results_inner.insert(
                        proxy.clone(),
                        dbg!(check_availability(&format!("https://{}", proxy)).await),
                    );
                }
                results.insert(site.clone(), results_inner);
            }
            dbg!(&results);
            client
                .put_mutable(MutableItem::new(
                    checker_key.clone(),
                    serde_json::to_string(&results).unwrap().into(),
                    gen_seq(),
                    None,
                ))
                .unwrap();

            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    });

    proxies_fetcher_worker.await.unwrap();
    proxies_checker_worker.await.unwrap();
}
