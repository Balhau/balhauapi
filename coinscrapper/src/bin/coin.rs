extern crate coinscrapper;
extern crate kafka;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

extern crate reqwest;
extern crate scraper;
extern crate tokio;

use std::fmt::Write;
use std::io::Read;
use std::time::Duration;

//Kafka dependencies
use kafka::producer::{Producer, Record, RequiredAcks};

//Local domains
use tokio::runtime::Runtime;


async fn process_markets_info(producer : &mut Producer) -> Result<(),reqwest::Error> {

    let c_market_link="https://api.coinmarketcap.com/v1/ticker";

    let body = reqwest::get(c_market_link)
        .await?
        .text()
        .await?;


        /*
    let markets_tick : Vec<CoinMarketEntry> = serde_json::from_str(body.as_str()).unwrap();
    let mut buf = String::with_capacity(2);

    markets_tick.iter().for_each(move |market|{
        let _ = write!(&mut buf,"{:?}",market);
        producer.send(&Record::from_value("coin.markets", buf.as_bytes())).unwrap();
        buf.clear();
    });

    */

    println!("Processed markets");

    Ok(())
}

async fn async_main() -> Result<(),reqwest::Error> {
    /*
    let js_str_parsed = JS_PAYLOAD.replace("24h_volume_usd", "_24h_volume_usd");
    let s: Vec<CoinMarketEntry> = serde_json::from_str(js_str_parsed.as_str()).unwrap();
    println!("Entry: {:?}", s);

    let c_market_link="https://api.coinmarketcap.com/v1/ticker";

    let body = reqwest::get(c_market_link)
        .await?
        .text()
        .await?;

    //println!("{:?}",resp);

    //Create kafka producer
    let mut producer =
        Producer::from_hosts(vec!("localhost:9092".to_owned()))
            .with_ack_timeout(Duration::from_secs(1))
            .with_required_acks(RequiredAcks::One)
            .create()
            .unwrap();

    /*
    let mut buf = String::with_capacity(2);
    let vec1 = markets_tick.iter().for_each(move |market|{
        let _ = write!(&mut buf,"{:?}",market);
        producer.send(&Record::from_value("coin.markets", buf.as_bytes())).unwrap();
        buf.clear();
    });
    */

    loop {
        process_markets_info(&mut producer);
        std::thread::sleep(Duration::from_secs(10));
    }
    /*
    for market in markets_tick.iter() {
        let r = write!(&mut buf,"{:?}",market);
        println!("{}",buf);
        println!("{}",counter);
        counter=counter+1;
    }
    */

    //let res = client.get("https://api.coinmarketcap.com/v1/ticker/".parse().unwrap())
    //    .get_result().unwrap();
    */
    Ok(())
}

fn main() {
    let mut r = Runtime::new().expect("Failed to Load Tokio Runtime");
    r.block_on(async_main());
}