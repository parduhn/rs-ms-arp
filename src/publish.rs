use crate::arp;
use crate::arp::models::{AppState, ArpResponses};
use pnet::datalink;
use std::sync::{Arc, Mutex};

pub fn start() {
    //vector of app states and interfaces
    let mut app_states: Vec<AppState> = Vec::new();

    for iface in datalink::interfaces() {
        let interface = iface.to_owned();
        if interface.is_loopback()
            || interface.ips.is_empty()
            || !interface.name.to_string().contains("enp")
                && !interface.name.to_string().contains("wlp")
        {
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

    if &app_states.len() > &0 {
        for app in &app_states {
            println!("Interface {:?}", app.interface.name);
            arp::arp::initiate_arp_handler(&app);
        }
    }
}
