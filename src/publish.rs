use crate::arp;
use crate::arp::models::{AppState, ArpResponse, ArpResponses};
use pnet::datalink;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};

pub fn start() {
    //vector of app states and interfaces
    let mut app_states: Vec<AppState> = Vec::new();

    for iface in datalink::interfaces() {
        let interface = iface.to_owned();
        let (tx, rx): (Sender<ArpResponse>, Receiver<ArpResponse>) = mpsc::channel();
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
                tx,
                rx,
            };
            app_states.push(app);
        }
    }

    if &app_states.len() > &0 {
        arp::arp::initiate_arp_handler(app_states);
    }
}
