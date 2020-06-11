extern crate serde;
extern crate serde_derive;
extern crate rweb;

use serde::Serialize;
use serde::Deserialize;
use rweb::*;


#[derive(Serialize, Deserialize, Debug)]
#[derive(Schema)]
pub struct YtsResponse {
    pub movies: Vec<MovieResponse>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[derive(Schema)]
pub struct MovieResponse {
    pub id: String,
    pub name: String,
    pub imdb_rate: String,
    pub url: String,
}
