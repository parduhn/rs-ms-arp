use crate::arp::arp::arp_handler;
use crate::arp::models::{AppState, ArpResponses};
// use actix_web::{http::Method, App};
use pnet::datalink;
use std::sync::{Arc, Mutex};

pub fn start() {
    loop {
        // let interfaces = datalink::interfaces()
        //     .iter()
        //     .filter(|ip| !ip.is_loopback() && !ip.ips.is_empty())
        //     .next()
        //     .unwrap()
        //     .to_owned();
        println!("----------------------------------------");
        for iface in datalink::interfaces() {
            let interface = iface.to_owned();
            if interface.is_loopback() || interface.ips.is_empty() {
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
                arp_handler(&app)
            }
        }
    }
}
