extern crate balhauapi;

use balhauapi::downloader::youtube::YoutubeDownloader;
use balhauapi::downloader::Downloadable;

fn main() {
    let movie = YoutubeDownloader {
        url: String::from("my lovely movie"),
        path: String::from("/tmp")
    };


    let res = movie.download();
    println!("Res: {}", res);
}
