#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate diesel;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate balhauapi;
extern crate rocket;

use rocket::http::ContentType;
use rocket::response::Response;
use rocket::request::Request;
use rocket::response::Result;
use rocket::response::Responder;
use std::io::Cursor;
use diesel::query_dsl::load_dsl::LoadDsl;

use balhauapi::db::api::*;
use balhauapi::db::schema::*;
use balhauapi::db::bookmarks::models::Bookmark;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}


#[derive(Serialize, Deserialize)]
pub struct About {
    name: String,
    email: String,
    country: String,
    desc: String,
    address: String
}

#[get("/bookmarks")]
fn get_bookmarks() -> String {
    let conn = create_conn();
    serde_json::to_string(&get_all_bookmarks(&conn)).unwrap()
}

#[get("/about")]
fn get_about() -> String {
    //Json(vec![t1,t2])
    let t1 = About {
        name: String::from("Kie"),
        email: String::from("bruce@wayne.com"),
        country: String::from("Tugings"),
        desc: String::from("Long, long, time ago... a big and tedious description follows"),
        address: String::from("221b Baker Street")
    };

    serde_json::to_string(&t1).unwrap()
}

fn main() {
    let routes = routes![
        index,
        get_about,
        get_bookmarks
    ];

    rocket::ignite()
        .mount("/", routes)
        .launch();
}
