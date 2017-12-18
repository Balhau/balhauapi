extern crate clap;

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("Bookmarks-cli")
        .arg(Arg::with_name("Config")
            .help("Set the configuration file for bookmarks-cli")
            .takes_value(true)
            .short("c")
            .long("config")
        )
        .subcommand(
            SubCommand::with_name("Load")
                .about("Load bookmarks into database")
                .version("1.0")
                .author("Balhau (balhau@balhau.net")
                .arg(Arg::with_name("input")
                    .help("The bookmark file to consume ")
                    .required(true)
                    .index(1)
                )
                .arg(
                    Arg::with_name("type")
                        .help("Type of bookmark to consume")
                        .takes_value(true)
                        .short("t")
                        .long("type")
                        .possible_values(&vec!["chrome", "firefox"])
                        .required(true)
                )
        ).get_matches();

    println!("{}",matches.usage.unwrap());
}