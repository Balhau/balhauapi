use super::Downloadable;
use std::process::Command;

pub struct YoutubeDownloader {
    pub url : String
}

/**
* Implement the trait for
*/
impl Downloadable<bool> for YoutubeDownloader {
    fn download(&self) -> bool {
        println!("Launching command");

        let output = Command::new("sh")
            .arg("-c")
            .arg("ls -lh")
            .output().expect(&self.url);

        println!("{}",String::from_utf8(output.stdout).unwrap());
        true
    }
}