extern crate balhauapi;
extern crate serde_yaml;

use balhauapi::downloader::youtube::YoutubeDownloader;
use balhauapi::downloader::Downloadable;

fn main() {
    let movie = YoutubeDownloader {
        url: String::from("https://www.youtube.com/watch?v=7OBGdwT8jsQ"),
        path: String::from("/tmp")
    };


    let res = movie.download();
    println!("Res: {}", res);
}
