extern crate pnet;
#[macro_use]
extern crate serde_derive;
use actix_web::{actix::System, server};
use nix::unistd::Uid;
use std::env;

mod api;
mod push;
mod router;

fn main() {
    if !Uid::effective().is_root() {
        panic!("You must run this executable with root permissions to scan network interfaces");
    }
    push::start();
    // let port = env::var("PORT").unwrap_or("4010".to_string());

    // let sys = System::new("ms-arp");
    // server::new(move || router::app_state())
    //     .bind(format!("0.0.0.0:{}", &port))
    //     .unwrap()
    //     .shutdown_timeout(2)
    //     .start();
    // println!("app started on port {}", port);
    // sys.run();
}