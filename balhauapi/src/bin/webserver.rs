#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate diesel;
extern crate rocket_contrib;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate balhauapi;
extern crate rocket;

use rocket::fairing::AdHoc;
use rocket_contrib::Json;
use rocket::config::{Config, Environment};

use balhauapi::db::api::*;
use balhauapi::confs::AppConfig;
use balhauapi::automation::remotecommand::reboot_server;
use balhauapi::confs::load_app_configuration;
use std::ops::Deref;


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

#[derive(Serialize, Deserialize)]
pub struct Machine {
    host: String
}

#[get("/bookmarks")]
fn get_bookmarks() -> String {
    let conn = create_conn();
    serde_json::to_string(&get_all_bookmarks(&conn)).unwrap()
}

#[get("/bookmarks/<page>/<max_page>")]
fn get_bookmarks_by_page(page : i64,max_page : i64) -> String {
    let conn = create_conn();
    serde_json::to_string(&get_bookmarks_paged(&conn,page*max_page,max_page)).unwrap()
}

#[post("/automation/reboot", format="application/json", data="<machine>")]
fn get_automation_reboot_host(machine : Json<Machine>) -> String {
    let app_config = load_app_configuration();
    let ref machine_obj: &Machine = machine.deref();

    let mut founded : bool = false;

    app_config.configs.remotes.iter().for_each(|item| {
        if item.name.eq(&machine_obj.host) {
            founded = true;
            reboot_server(item);
        }
    });

    match founded {
        true => String::from("Machine: ")+&machine_obj.host+" rebooted ",
        _   => String::from("machine not found")
    }
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

    let app_configs : AppConfig = load_app_configuration();
    println! ("Configurations:\n {:?}",&app_configs);

    let routes = routes![
        index,
        get_about,
        get_bookmarks,
        get_bookmarks_by_page,
        get_automation_reboot_host
    ];

    let env_type : Environment = match app_configs.configs.webserver.env.as_str() {
        "prod"  => Environment::Production,
        _       => Environment::Staging
    };



    let rocket_config = Config::build(env_type)
        .address(app_configs.configs.webserver.binding_host)
        .port(app_configs.configs.webserver.port)
        .finalize().expect("Error building webserver configuration object");


    rocket::custom(rocket_config,app_configs.configs.webserver.log)
        .mount("/", routes)
        .attach(AdHoc::on_response(|_,resp|{
            resp.set_raw_header("Access-Control-Allow-Origin","*");
        }))
        .launch();
}
