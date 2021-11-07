use crate::api::arp::arp_handler_push;
use crate::api::models::{AppState, ArpResponses};
// use actix_web::{http::Method, App};
use pnet::datalink;
use std::sync::{Arc, Mutex};

pub fn start() {
    let app = AppState {
        knowns: Arc::new(Mutex::new(ArpResponses {
            results: Vec::new(),
        })),
        interface: datalink::interfaces()
            .iter()
            .filter(|ip| !ip.is_loopback() && !ip.ips.is_empty())
            .next()
            .unwrap()
            .to_owned(),
    };
    arp_handler_push(app)
}
