extern crate reqwest;
extern crate tokio;

use futures::executor::block_on;
use tokio::runtime::Runtime;

async fn hello_world() {
    println!("hello, world!");
}

fn main() {
    let mut r = Runtime::new().expect("Failed to load Tokio runtime");
    let futureFuture = hello_world(); // Nothing is printed
    let stuffFuture = cenas();
    r.block_on(futureFuture);
    r.block_on(stuffFuture);
}

async fn cenas() -> Result<(), reqwest::Error> {
  let res = reqwest::get("https://hyper.rs").await?;

  println!("Status: {}", res.status());

  let body = res.text().await?;

  println!("Body:\n\n{}", body);

  Ok(())
}