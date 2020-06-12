extern crate serde;
extern crate serde_derive;

use serde::Serialize;
use serde::Deserialize;


#[derive(Serialize, Deserialize, Debug)]
pub struct YtsResponse {
    pub movies: Vec<MovieResponse>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MovieResponse {
    pub id: String,
    pub name: String,
    pub imdb_rate: String,
    pub url: String,
}
