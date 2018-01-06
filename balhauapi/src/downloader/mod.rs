pub mod youtube;
pub mod processor;

use downloader::youtube::YoutubeDownloader;

/**
* Dictionary with all the Downloader implementations
*/
pub enum Downloaders{
    Youtube(YoutubeDownloader)
}

pub trait Downloadable {
    fn download(&self) -> bool;
}
