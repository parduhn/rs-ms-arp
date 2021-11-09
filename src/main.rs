extern crate pnet;
extern crate serde_derive;

use ms_arp_lib::push;
use nix::unistd::Uid;

fn main() {
    if !Uid::effective().is_root() {
        panic!("You must run this executable with root permissions to scan network interfaces");
    }
    push::start();
}
