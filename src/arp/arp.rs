use pnet::datalink::{self, Channel, MacAddr, NetworkInterface};
use std::net::{IpAddr, Ipv4Addr};
use std::sync::mpsc::{Receiver, SendError, Sender};
use std::{env, thread, time::Duration};

use pnet::packet::arp::MutableArpPacket;
use pnet::packet::ethernet::MutableEthernetPacket;
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::{MutablePacket, Packet};

use pnet::packet::arp::{ArpHardwareTypes, ArpOperation, ArpOperations, ArpPacket};

use crate::arp::macvendor::vendor_request;
use crate::arp::models::{AppState, ArpResponse, ArpResponses};
use crate::arp::mq;
use ipnetwork::IpNetwork;

fn send_arp_packet(
    interface: NetworkInterface,
    source_ip: Ipv4Addr,
    source_mac: MacAddr,
    target_ip: Ipv4Addr,
) {
    let (mut tx, _) = match datalink::channel(&interface, Default::default()) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unknown channel type"),
        Err(e) => panic!("Error happened {}", e),
    };
    let mut ethernet_buffer = [0u8; 42];
    let mut ethernet_packet = MutableEthernetPacket::new(&mut ethernet_buffer).unwrap();

    ethernet_packet.set_destination(MacAddr::broadcast());
    ethernet_packet.set_source(source_mac);
    ethernet_packet.set_ethertype(EtherTypes::Arp);

    let mut arp_buffer = [0u8; 28];
    let mut arp_packet = MutableArpPacket::new(&mut arp_buffer).unwrap();

    arp_packet.set_hardware_type(ArpHardwareTypes::Ethernet);
    arp_packet.set_protocol_type(EtherTypes::Ipv4);
    arp_packet.set_hw_addr_len(6);
    arp_packet.set_proto_addr_len(4);
    arp_packet.set_operation(ArpOperations::Request);
    arp_packet.set_sender_hw_addr(source_mac);
    arp_packet.set_sender_proto_addr(source_ip);
    arp_packet.set_target_hw_addr(MacAddr::zero());
    arp_packet.set_target_proto_addr(target_ip);

    ethernet_packet.set_payload(arp_packet.packet_mut());
    tx.send_to(ethernet_packet.packet(), Some(interface));
}

pub fn recv_arp_packets(interface: NetworkInterface, tx: Sender<ArpResponse>) {
    thread::spawn(move || {
        let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
            Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => panic!("Unknown channel type"),
            Err(e) => panic!("Error happened {}", e),
        };

        loop {
            match rx.next() {
                Ok(data) => {
                    let ethernet_packet = EthernetPacket::new(data).unwrap();
                    let ethernet_payload = ethernet_packet.payload();
                    let arp_packet = ArpPacket::new(ethernet_payload).unwrap();
                    let arp_reply_op = ArpOperation::new(2_u16);

                    if arp_packet.get_operation() == arp_reply_op {
                        let result = ArpResponse {
                            mac_addr: arp_packet.get_sender_hw_addr().to_string(),
                            ip4: arp_packet.get_sender_proto_addr().to_string(),
                            vendor_name: "".to_string(),
                        };
                        // tx.send(result);
                        match tx.send(result) {
                            Ok(()) => (),
                            Err(SendError(_e)) => (),
                        }
                    }
                }
                Err(e) => panic!("An error occurred while reading packet: {}", e),
            }
        }
    });
}

//set up a channel, call send and recv
pub fn get_arp_results(
    interface: NetworkInterface,
    knowns: &mut ArpResponses,
    rx: &Receiver<ArpResponse>,
) -> Vec<ArpResponse> {
    //optional variable to add a comma-separated list of known mac addresses to ignore from displaying
    let ignores = env::var("IGNORE").unwrap_or_default();
    let ignores_vec: Vec<&str> = ignores.split(",").collect();

    //the url for the api that returns vendor information from the mac addr
    let vendor_url = env::var("MACVENDOR_URL").unwrap_or("https://api.macvendors.com".to_string());
    let source_mac = match interface.mac {
        Some(m) => m,
        None => panic!("Error in mac"),
    };

    let source_network = interface.ips.iter().find(|ip| ip.is_ipv4()).unwrap();
    let source_ip = source_network.ip();

    let mut sent = 0;
    match source_network {
        //for mac development I had to set ipv6 to manual
        &IpNetwork::V4(source_networkv4) => {
            if let IpAddr::V4(source_ipv4) = source_ip {
                for target_ipv4 in source_networkv4.iter() {
                    send_arp_packet(interface.clone(), source_ipv4, source_mac, target_ipv4);
                    sent += 1;
                }
            } else {
                println!("source ip was not ipv4");
            }
        }
        e => {
            println!("Error while attempting to get network for interface: {}", e);
        }
    }
    let mut arp_list: Vec<ArpResponse> = Vec::new();
    for _ in 0..sent {
        match rx.try_recv() {
            Ok(arp_res) => {
                // println!("{:?}", arp_res);
                arp_list.push(arp_res)
            }
            Err(_) => break,
        }
    }

    for m in arp_list {
        let mut device = ArpResponse {
            mac_addr: m.mac_addr.to_string(),
            vendor_name: "unkown".to_string(),
            ip4: m.ip4.clone(),
        };

        let short_mac = &m.mac_addr.to_string()[..8];
        if !ignores_vec.contains(&short_mac) && !knowns.results.contains(&device) {
            thread::sleep(Duration::from_secs(1));
            match vendor_request(&vendor_url, short_mac) {
                Ok(s) => {
                    device.vendor_name = s.clone();
                    knowns.results.push(device.clone());
                }
                Err(e) => {
                    println!("Error on {:?}", e);
                }
            }
        } else {
            let index = knowns.results.iter().position(|r| r == &device).unwrap();
            match knowns.results.get(index) {
                Some(arp) => device.vendor_name = arp.vendor_name.clone(),
                None => device.vendor_name = "stillunkown".to_string(),
            }
        }
    }
    knowns.results.clone()
}

pub fn initiate_arp_handler(app_states: Vec<AppState>) {
    println!("App states {:?}", &app_states);

    for state in &app_states {
        //start channel to listen
        let iface = state.interface.clone();
        recv_arp_packets(iface.clone(), state.tx.clone());
    }

    //loop with sending arp scan
    loop {
        for state in &app_states {
            let iface = state.interface.clone();
            match state.knowns.lock() {
                Ok(mut k) => {
                    //read list of knowns,
                    //if a mac addr on local network is not in list of knowns, call vendor api, then store results from api back into knowns
                    println!(
                        "---------------------------------------- Interface {:?}",
                        state.interface.name
                    );
                    let mut response = Vec::new();
                    response = get_arp_results(iface.clone(), &mut k, &state.rx);
                    for device in &response {
                        mq::send(device);
                        thread::sleep(Duration::from_secs(1));
                    }
                }
                Err(e) => {
                    println!("error obtaining mutex lock: {}", e);
                    // HttpResponse::InternalServerError().finish()
                }
            }
        }
    }
}
