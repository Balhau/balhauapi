extern crate clap;
extern crate select;
extern crate balhauapi;
extern crate rusqlite;

use clap::App;//, Arg, SubCommand};
use rusqlite::{params, Connection, Row};
use balhauapi::db::ereader::models::Item;
use balhauapi::db::api::save_ereader_item;
use balhauapi::db::api::create_conn;
use balhauapi::db::api::truncate_ereaderitem_table;


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
        params![],| _row : &Row | {
            Result::Ok(Item{
                id_item: 0,
                f_id_item : _row.get(0)?,
                f_pages_number: _row.get(7)?,
                f_current_page: _row.get(8)?,
                f_last_read: _row.get(9)?,
                f_publication_date: _row.get(10)?,
                f_publisher: _row.get(11)?,
                f_title: _row.get(12)?,
                f_description: _row.get(14)?,
                f_author_id: _row.get(22)?
            })
        }
    ).unwrap();

    let conn = create_conn();

    truncate_ereaderitem_table(&conn);

    for book in sel_iter {
        let item = book.unwrap();
        save_ereader_item(&conn,&item);
        println! ("Found book {:?}",&item);
    }

    println!("{:?}",matches.usage.unwrap())
}