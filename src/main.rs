extern crate pnet;
#[macro_use]
extern crate serde_derive;
use actix_web::{actix::System, server};
use std::env;

mod api;
mod router;

fn main() {
    let port = env::var("PORT").unwrap_or("8081".to_string());

    let sys = System::new("arp-microsvc");
    server::new(move || router::app_state())
        .bind(format!("0.0.0.0:{}", &port))
        .unwrap()
        .shutdown_timeout(2)
        .start();
    println!("app started on port {}", port);
    sys.run();

}
