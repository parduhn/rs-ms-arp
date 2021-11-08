extern crate pnet;
#[macro_use]
extern crate serde_derive;
use nix::unistd::Uid;
mod api;
mod push;

fn main() {
    if !Uid::effective().is_root() {
        panic!("You must run this executable with root permissions to scan network interfaces");
    }
    push::start();
}
