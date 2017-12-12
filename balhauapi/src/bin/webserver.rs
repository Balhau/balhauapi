#![feature(plugin)]
#![plugin(rocket_codegen)]


extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate rocket;

//use rocket::http::ContentType;
use rocket::response::Response;
use rocket::request::Request;
use rocket::response::Result;
use rocket::response::Responder;
use std::io::Cursor;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}


#[derive(Serialize, Deserialize)]
struct About {
    name: String,
    email: String,
    country: String,
    desc: String,
    address: String
}

impl<'r> Responder<'r> for About {
    fn respond_to(self, _: &Request) -> Result<'r> {
        Response::build()
            .sized_body(Cursor::new(format!("{}",serde_json::to_string(&self).unwrap())))
            .raw_header("Access-Control-Allow-Origin", String::from("*"))
            //.header(ContentType::new("application", "application/json"))
            .ok()
    }
}

#[get("/about")]
fn get_about() -> About {
    //Json(vec![t1,t2])
    let t1 = About{
        name : String::from("Kie"),
        email: String::from("bruce@wayne.com"),
        country : String::from("Tugings"),
        desc : String::from("Long, long, time ago... a big and tedious description follows"),
        address: String::from("221b Baker Street")
    };

    t1
}

fn main() {
    let routes = routes![
        index,
        get_about
    ];

    rocket::ignite().mount("/", routes).launch();
}
