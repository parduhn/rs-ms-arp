use pnet::datalink::NetworkInterface;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize)]
pub struct ArpResponse {
    pub mac_addr: String,
    pub vendor_name: String,
    pub ip4: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ArpResponses {
    pub results: Vec<ArpResponse>,
}

#[derive(Debug)]
pub struct AppState {
    pub knowns: Arc<Mutex<ArpResponses>>,
    pub interface: NetworkInterface,
    pub tx: Sender<ArpResponse>,
    pub rx: Receiver<ArpResponse>,
}

impl PartialEq for ArpResponse {
    fn eq(&self, other: &ArpResponse) -> bool {
        self.mac_addr == other.mac_addr && self.ip4 == other.ip4
    }
}
