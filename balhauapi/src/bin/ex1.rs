extern crate balhauapi;
extern crate serde_yaml;
extern crate ssh2;

use balhauapi::downloader::youtube::YoutubeDownloader;
//use balhauapi::downloader::Downloadable;
use balhauapi::confs::SshContext;
use balhauapi::automation::execute_ssh_command;
use balhauapi::automation::remotecommand::reboot_server;
use balhauapi::confs::load_app_configuration;


fn main() {
    let _movie = YoutubeDownloader {
        url: String::from("https://www.youtube.com/watch?v=7OBGdwT8jsQ"),
        path: String::from("/tmp")
    };


    //let res = movie.download();
    //println!("Res: {}", res);

    let app_conf = load_app_configuration();

    let ssh_context = app_conf.configs.remotes.get(0).unwrap();

    let result = execute_ssh_command(String::from("ls -lh"), ssh_context);

    println! ("{}",&result);

    //let result = reboot_server(&ssh_context);

    //println! ("{}",&result);
}
