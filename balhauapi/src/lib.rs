#![recursion_limit="4096"]
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_infer_schema;
#[macro_use] extern crate serde_derive;


extern crate dotenv;

pub mod db;
pub mod downloader;
pub mod confs;


