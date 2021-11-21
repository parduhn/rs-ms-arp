use crate::arp::arp::arp_handler;
use crate::arp::models::{AppState, ArpResponses};
// use actix_web::{http::Method, App};
use pnet::datalink;
use std::sync::{Arc, Mutex};

pub fn start() {
    let interfaces = datalink::interfaces()
        .iter()
        .filter(|ip| !ip.is_loopback() && !ip.ips.is_empty())
        .next()
        .unwrap()
        .to_owned();

    println!("{:?}", interfaces);
    println!("----------------------------------------");
    for iface in datalink::interfaces() {
        let interface = iface.to_owned();
        if interfaces.is_loopback() || interfaces.ips.is_empty() {
            println!("Not using: {:?}", interface);
        } else {
            println!("Using: {:?}", interface);
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
            loop {
                arp_handler(&app)
            }
        }
    }
}
