#![feature(proc_macro_hygiene, decl_macro)]

extern crate diesel;
extern crate rocket_contrib;
extern crate serde_json;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket;

extern crate balhauapi;

use rocket::fairing::AdHoc;
use rocket::State;
use rocket_contrib::json::Json;
use rocket::config::{Config, Environment};

use std::sync::Mutex;

use balhauapi::downloader::youtube::YoutubeDownloader;
use balhauapi::downloader::Downloadable;
use balhauapi::downloader::processor::DownloaderProcessor;
use balhauapi::downloader::processor::Processor;
use balhauapi::downloader::Downloaders;
use balhauapi::db::api::*;
use balhauapi::confs::AppConfig;
use balhauapi::confs::msgs::Messages;
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
    address: String,
}

#[derive(Serialize, Deserialize)]
pub struct Machine {
    host: String
}

#[derive(Serialize, Deserialize)]
pub struct DownloadMedia {
    media_type: String,
    url: String,
}

#[get("/bookmarks")]
fn get_bookmarks() -> String {
    let conn = create_conn();
    let b = get_all_bookmarks(&conn);
    serde_json::to_string(&b).unwrap()
}

#[get("/bookmarks/<page>/<max_page>")]
fn get_bookmarks_by_page(page: i64, max_page: i64) -> String {
    let conn = create_conn();
    serde_json::to_string(&get_bookmarks_paged(&conn, page * max_page, max_page)).unwrap()
}

#[post("/downloader/video", format = "application/json", data = "<media>")]
fn post_downloader_video(media: Json<DownloadMedia>,downloader : State<Mutex<DownloaderProcessor>>) -> String {
    let app_config: AppConfig = load_app_configuration();
    let media_obj = media.deref();

    match media_obj.media_type.as_str() {
        "Youtube" => {
            let movie = YoutubeDownloader {
                url: String::from(media_obj.url.as_str()),
                path: app_config.configs.folders.mediafolder,
            };

            downloader
                .lock()
                .unwrap()
                .process(Downloaders::Youtube(movie));

            String::from("Downloaded youtube video")
        }
        _ => String::from("Media type not found")
    }
}

#[post("/automation/reboot", format = "application/json", data = "<machine>")]
fn post_automation_reboot_host(machine: Json<Machine>) -> String {
    let app_config = load_app_configuration();
    let machine_obj = machine.deref();

    let mut founded: bool = false;

    app_config.configs.remotes.iter().for_each(|item| {
        if item.name.eq(&machine_obj.host) {
            founded = true;
            reboot_server(item);
        }
    });

    match founded {
        true => String::from("Machine: ") + &machine_obj.host + " rebooted ",
        _ => String::from("machine not found")
    }
}

#[get("/about")]
fn get_about() -> String {
    let t1 = About {
        name: String::from("Kie"),
        email: String::from("bruce@wayne.com"),
        country: String::from("Tugings"),
        desc: String::from("Long, long, time ago... a big and tedious description follows"),
        address: String::from("221b Baker Street"),
    };

    serde_json::to_string(&t1).unwrap()
}

fn main() {
    let app_configs: AppConfig = load_app_configuration();
    //println!("Configurations:\n {:?}", &app_configs);

    let routes = routes![
        index,
        get_about,
        get_bookmarks,
        get_bookmarks_by_page,
        post_automation_reboot_host,
        post_downloader_video
    ];

    let env_type: Environment = match app_configs.configs.webserver.env.as_str() {
        "prod" => Environment::Production,
        _ => Environment::Staging
    };

    let rocket_config = Config::build(env_type)
        .address(app_configs.configs.webserver.binding_host)
        .port(app_configs.configs.webserver.port)
        .finalize().expect(Messages::ERROR_SERVER_CONFIGS);


    rocket::custom(rocket_config)
        .mount("/", routes)
        .manage(Mutex::new(DownloaderProcessor::new()))
        .attach(AdHoc::on_response("Dummy",|req, resp| {
            resp.set_raw_header("Access-Control-Allow-Origin", "*");
        }))
        .launch();
}
