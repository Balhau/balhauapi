extern crate clap;

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("Bookmarks-cli")
        .about("\nbookmarks-cli is a command line interface for bookmarks management.

Examples:

    Load a list of bookmarks from google chrome
        bookmarks load -t chrome bookmarks.html

"
        )
        .arg(Arg::with_name("Config")
            .help("Set the configuration file for bookmarks-cli")
            .takes_value(true)
            .short("c")
            .long("config")
        )
        .subcommand(
            SubCommand::with_name("load")
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

    //Validate load command
    if matches.is_present("load") {
        if !matches.is_present("input") {
            println!("You should provide an input file, type <command> help for more information");
        }
        if !matches.is_present("type") {
            println!("You should provide the type of bookmark to process, type <command> help for more information");
        }
    } else {
        // Change this as soon as more commands are implemented
        println!("At the moment only load command is available, type <command> help for more information ");
    }
}