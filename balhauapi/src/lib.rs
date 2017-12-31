#![recursion_limit="4096"]

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_infer_schema;
#[macro_use] extern crate serde_derive;


extern crate dotenv;
extern crate ssh2;

pub mod db;
pub mod downloader;
pub mod confs;
pub mod automation;
pub mod bookmarks;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}



