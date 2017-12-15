use super::Downloadable;
use std::process::Command;

pub struct YoutubeDownloader {
    pub url : String,
    pub path: String
}

/**
* Implement the trait Downloadable for the YoutubeDownloader struct
*/
impl Downloadable<bool> for YoutubeDownloader {
    fn download(&self) -> bool {
        println!("Fetching video: {} into {}",self.url,self.path);

        let mut command = "".to_string();
        let splited : Vec<&str> = self.url.split("&").collect();
        let parsed_url = splited.get(0).unwrap();

        command.push_str("cd ");
        command.push_str(&self.path);
        command.push_str("; youtube-dl ");
        command.push_str("\"");
        command.push_str(parsed_url);
        command.push_str("\"");

        let output = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .output().expect(&self.url);
        //Print command executed
        println!("Command executed: {}", command);
        println!("Result: {:?}",output.stdout);
        true
    }
}
