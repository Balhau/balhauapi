extern crate balhauapi;
extern crate serde_yaml;
extern crate ssh2;

use balhauapi::downloader::youtube::YoutubeDownloader;
use balhauapi::downloader::Downloadable;
use balhauapi::downloader::Downloaders;
use balhauapi::downloader::processor::DownloaderProcessor;
use balhauapi::downloader::processor::Processor;
use balhauapi::confs::SshContext;
use std::thread::JoinHandle;
use balhauapi::automation::execute_ssh_command;
use balhauapi::automation::remotecommand::reboot_server;
use balhauapi::confs::load_app_configuration;


fn main() {
    /*
    let movie1 = YoutubeDownloader {
        url: String::from("https://www.youtube.com/watch?v=7OBGdwT8jsQ"),
        path: String::from("/tmp")
    };

    let movie2 = YoutubeDownloader {
        url: String::from("https://www.youtube.com/watch?v=svhrZFh78W4"),
        path: String::from("/tmp")
    };



    let processor = DownloaderProcessor::new();

    processor.process(Downloaders::Youtube(movie1));
    processor.process(Downloaders::Youtube(movie2));

    processor.wait();
    */


    //let res = movie.download();
    //println!("Res: {}", res);

    //let app_conf = load_app_configuration();

    //let ssh_context = app_conf.configs.remotes.get(0).unwrap();

    //let result = execute_ssh_command(String::from("ls -lh"), ssh_context);

    //println! ("{}",&result);

    //let result = reboot_server(&ssh_context);

    //println! ("{}",&result);
}
