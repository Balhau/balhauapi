extern crate clap;
extern crate select;
extern crate balhauapi;

use clap::{App, Arg, SubCommand};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let matches = App::new("EReader-cli")
        .about("
ereader-cli is a command line interface for ereader metadata management.

Examples:

    Import metadata from cybook ocean eraeder sqlite database
        ereader import -t ocean <sqlitedb>
"
        )
        .author("Balhau <balhau@balhau.net>")
        .version("0.1")
        .get_matches();

    println! ("Cenas");
}