use super::Downloadable;
use super::Downloaders;
use super::Downloaders::Youtube;
use std::thread;
use std::thread::Thread;
use std::boxed::Box;
use std::thread::JoinHandle;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::sync::{Arc,Mutex};
/**
* This will serve as a multithreading strategy to receive downloading calls and process them asynchronously
*/

const THREAD_POOL_SIZE : usize = 10;

pub struct DownloaderProcessor{
    tx : Sender<Downloaders>,
    t : JoinHandle<()>
}

pub trait Processor {
    fn process(&self, item : Downloaders);
}


impl DownloaderProcessor {
    pub fn new() -> DownloaderProcessor {
        let (tx,rx) = channel();

        let th : JoinHandle<()> = thread::spawn(move || {
            loop {
                let d: Downloaders = rx.recv().unwrap();
                match d {
                    Youtube(yd) => yd.download()
                };
            }
        });

        DownloaderProcessor{
            tx,
            t: th
        }
    }

    pub fn wait(self) {
        self.t.join();
    }
}

impl  Processor for DownloaderProcessor{

    fn process(&self, item: Downloaders) {
        self.tx.send(item);
    }

}
