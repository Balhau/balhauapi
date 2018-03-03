extern crate serde_yaml;
extern crate balhauapi;

use balhauapi::confs::*;

fn main() {
    println! ("{:?}",load_app_configuration())
}