mod ussd;
use ussd::*;

mod poap;
use poap::*;

mod messaging;
use messaging::*;

fn main() {
    start_server();
}
