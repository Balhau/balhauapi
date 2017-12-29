extern crate clap;
extern crate select;
extern crate balhauapi;
extern crate rusqlite;

use clap::App;//, Arg, SubCommand};
use rusqlite::Connection;
use balhauapi::db::ereader::models::Item;
use balhauapi::db::api::save_ereader_item;
use balhauapi::db::api::create_conn;


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
    let path = "/home/vitorfernandes/Documents/ereader/card/system/library";
    let conn = Connection::open(path).unwrap();

    let mut stmt = conn.prepare("SELECT * from T_ITEM;").unwrap();

    let sel_iter = stmt.query_map(
        &[],| row | {
            Item{
                id_item: 0,
                f_id_item : row.get(0),
                f_pages_number: row.get_checked(7).unwrap_or(Option::from(0)),
                f_current_page: row.get_checked(8).unwrap_or(Option::from(0)),
                f_last_read: row.get_checked(9).unwrap_or(Option::from(0)),
                f_publication_date: row.get_checked(10).unwrap_or(Option::from(String::from(""))),
                f_publisher: row.get_checked(11).unwrap_or(Option::from(String::from(""))),
                f_title: row.get_checked(12).unwrap_or(Option::from(String::from(""))),
                f_description: row.get_checked(14).unwrap_or(Option::from(String::from(""))),
                f_author_id: row.get_checked(22).unwrap_or(Option::from(0))
            }
        }
    ).unwrap();

    let conn = create_conn();

    for book in sel_iter {
        let item = book.unwrap();
        save_ereader_item(&conn,&item);
        println! ("Found book {:?}",&item);
    }

    println!("{:?}",matches.usage.unwrap())
}