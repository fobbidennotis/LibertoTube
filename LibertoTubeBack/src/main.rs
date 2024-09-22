use axum::{extract::State, Router};
use dotenv::dotenv;
use ed25519_dalek::VerifyingKey;
use mainline::{Dht, SigningKey};
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::sync::RwLock;
use utils::{from_hex, get_message, to_hex};

mod checker;
mod controller;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let args: Vec<_> = std::env::args().collect();
    // TODO: dry this code up a bit
    match &args[1][..] {
        "gen-keypair" => {
            let key = ed25519_dalek::SigningKey::generate(&mut rand::rngs::OsRng);
            println!("Pubkey: {}", to_hex(key.verifying_key().as_bytes()));
            println!("Privkey: {}", to_hex(key.as_bytes()));
        }
        "set-proxies" => {
            let proxies_filename = &args[2];
            let proxies_json = std::fs::read_to_string(proxies_filename).unwrap();
            let proxies = serde_json::from_str(&proxies_json).unwrap();
            let key = utils::from_hex(&std::env::var("PROXIES_CONTROLLER_KEY").unwrap())
                .try_into()
                .unwrap();
            controller::set_proxies(SigningKey::from_bytes(&key), proxies).await;
        }
        "set-checkers" => {
            let checkers_filename = &args[2];
            let checkers_json = std::fs::read_to_string(checkers_filename).unwrap();
            let checkers = serde_json::from_str(&checkers_json).unwrap();
            let key = utils::from_hex(&std::env::var("CHECKERS_CONTROLLER_KEY").unwrap())
                .try_into()
                .unwrap();
            controller::set_checkers(SigningKey::from_bytes(&key), checkers).await;
        }
        "checker" => {
            let announcer_pubkey =
                utils::from_hex(&std::env::var("PROXIES_CONTROLLER_PUBKEY").unwrap())
                    .try_into()
                    .unwrap();
            let key = utils::from_hex(&std::env::var("CHECKER_KEY").unwrap())
                .try_into()
                .unwrap();
            checker::checker_server(
                VerifyingKey::from_bytes(&announcer_pubkey).unwrap(),
                SigningKey::from_bytes(&key),
            )
            .await;
        }
        "node" => {
            node_server().await;
        }
        _ => unreachable!(),
    }
}

async fn node_server() {
    let checkers: Arc<RwLock<Vec<String>>> = Arc::new(RwLock::new(Vec::new()));
    tokio::spawn(checkers_fetcher_worker(checkers.clone()));

    let proxies: Arc<RwLock<HashMap<String, Vec<String>>>> = Arc::new(RwLock::new(HashMap::new()));
    tokio::spawn(checkers_result_fetcher_worker(
        checkers.clone(),
        proxies.clone(),
    ));

    let app = Router::new()
        .route("/", axum::routing::get(get_proxies))
        .layer(tower_http::cors::CorsLayer::new().very_permissive())
        .with_state(proxies.clone());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:7777").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_proxies(State(proxies): State<Arc<RwLock<HashMap<String, Vec<String>>>>>) -> String {
    serde_json::to_string(&*proxies.read().await).unwrap()
}

async fn checkers_fetcher_worker(checkers: Arc<RwLock<Vec<String>>>) {
    let client = Dht::client().unwrap();
    let checkers_controller_pubkey =
        utils::from_hex(&std::env::var("CHECKERS_CONTROLLER_PUBKEY").unwrap())
            .try_into()
            .unwrap();
    let mut last_seq = 0;
    loop {
        let Some(msg) = get_message(&client, &checkers_controller_pubkey) else {
            continue;
        };
        if *msg.seq() > last_seq {
            *checkers.write().await =
                serde_json::from_str(std::str::from_utf8(msg.value()).unwrap()).unwrap();
            last_seq = *msg.seq();
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

// FIXME
async fn checkers_result_fetcher_worker(
    checkers: Arc<RwLock<Vec<String>>>,
    proxies: Arc<RwLock<HashMap<String, Vec<String>>>>,
) {
    let client = Dht::client().unwrap();
    loop {
        let mut speeds: HashMap<String, HashMap<String, Vec<usize>>> = HashMap::new();
        for checker in dbg!(checkers.read().await).iter() {
            let Some(msg) = get_message(&client, &from_hex(checker).try_into().unwrap()) else {
                continue;
            };
            let checker_result: HashMap<String, HashMap<String, usize>> =
                serde_json::from_str(dbg!(std::str::from_utf8(msg.value()).unwrap())).unwrap();
            for (site, proxies) in checker_result.iter() {
                for (proxy, speed) in proxies.iter() {
                    speeds
                        .entry(site.to_string()) // TODO put this into an Arc
                        .or_insert_with(|| HashMap::new())
                        .entry(proxy.clone())
                        .or_insert_with(|| Vec::new())
                        .push(*speed);
                }
            }
        }

        *proxies.write().await = speeds
            .into_iter()
            .map(|(site, proxies)| {
                let mut proxies: Vec<_> = proxies
                    .into_iter()
                    .map(|(proxy, speeds)| {
                        let sum: usize = speeds.iter().sum();
                        (proxy, sum / speeds.len())
                    })
                    .filter(|(_proxy, speed)| *speed != 0)
                    .take(3)
                    .collect();
                proxies.sort_by_key(|(_k, v)| *v);
                (site, proxies.into_iter().map(|(k, _v)| k).collect())
            })
            .collect::<HashMap<_, _>>();
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
