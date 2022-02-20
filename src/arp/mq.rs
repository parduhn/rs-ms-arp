use crate::arp::models::ArpResponse;
use std::{thread, time::Duration};
//         let device_string = serde_json::to_string(&device).unwrap();

fn ip_to_string(arp_response: &ArpResponse) -> String {
    let result = serde_json::to_string(&arp_response).unwrap();
    result
}

pub fn send(device: &ArpResponse) {
    let par = lib_mq::build_topic_parameter("amqp://timeover:timeover@localhost:5672", "arp");

    let msg = ip_to_string(device);
    let result = lib_mq::send(&par, &msg);
    println!("Message {:?}", &msg);

    if !result.is_ok() {
        println!("MessageQueue error {:?}", result);
        thread::sleep(Duration::from_secs(10));
    }
}
