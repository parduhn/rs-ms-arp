use crate::arp::arp::arp_handler;
use crate::arp::models::{AppState, ArpResponses};
// use actix_web::{http::Method, App};
use pnet::datalink;
use std::sync::{Arc, Mutex};

pub fn start() {
    //vector of app states and interfaces
    let mut app_states: Vec<AppState> = Vec::new();

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
                interface,
            };
            app_states.push(app);
        }
    }

    loop {
        println!("----------------------------------------");
        for app in &app_states {
            arp_handler(&app)
        }
    }
}
