#![recursion_limit="128"]
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_infer_schema;


extern crate dotenv;

pub mod db;
pub mod downloader;

use diesel::prelude::*;
use diesel::types::Timestamp;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;


