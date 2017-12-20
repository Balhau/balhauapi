extern crate clap;
extern crate select;
extern crate balhauapi;

use clap::{App, Arg, SubCommand};
use std::fs::File;
use std::io::prelude::*;

use select::document::Document;
use select::predicate::Name;
use std::time::SystemTime;
use std::time::Duration;
use std::time::UNIX_EPOCH;
use balhauapi::db::api::*;

fn read_bookmark_file_chrome(path: &str) -> String {
    let mut f = File::open(path).expect("File not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Error reading the file");

    contents
}

fn main() {
    let matches = App::new("Bookmarks-cli")
        .about("
bookmarks-cli is a command line interface for bookmarks management.

Examples:

    Load a list of bookmarks from google chrome
        bookmarks save -t chrome bookmarks.html

"
        )
        .arg(Arg::with_name("Config")
            .help("Set the configuration file for bookmarks-cli")
            .takes_value(true)
            .short("c")
            .long("config")
        )
        .subcommand(
            SubCommand::with_name("save")
                .about("Save bookmarks into database")
                .version("1.0")
                .author("Balhau (balhau@balhau.net")
                .arg(Arg::with_name("input")
                         .help("The bookmark file to consume ")
                         .required(true)
                     //.index(2)
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

    //Validate save command
    if matches.is_present("save") {
        //Process command no need for
    } else {
        // Change this as soon as more commands are implemented
        println!("At the moment only <save> command is available, type help for more information ");
        std::process::exit(1);
    }

    if let Some(ref subcommand_matches) = matches.subcommand_matches("save") {
        let conn = create_conn();
        let input_path: &str = subcommand_matches.args.get("input").unwrap().vals[0].to_str().unwrap();
        let bookmark_type: &str = subcommand_matches.args.get("type").unwrap().vals[0].to_str().unwrap();
        //Process load command
        println!("Saving bookmarks {} from {}",
                 input_path,
                 bookmark_type);

        let html: String = read_bookmark_file_chrome(input_path);

        for item in Document::from(html.as_str()).find(Name("dt")) {
            //Process bookmark entry
            for entry in item.find(Name("a")) {
                let href = entry.attr("href").unwrap();
                let date_str = entry.attr("add_date").unwrap();
                let duration = date_str.parse::<u64>().unwrap();
                let date: SystemTime = UNIX_EPOCH + Duration::from_secs(duration);
                let b64icon : &str = entry.attr("icon").get_or_insert("");
                let desc = entry.inner_html();

                let bookmark = save_bookmark(
                    &conn,
                    &desc,
                    &b64icon,
                    &href,
                    &date
                );

                println!("Saved Bookmark: {:?}", bookmark);
            }
        }
    }

    std::process::exit(0);
}