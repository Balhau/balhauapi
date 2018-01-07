extern crate coinscrapper;
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate kafka;

extern crate scraper;
extern crate reqwest;

use std::io::Read;
use std::fmt::Write;
use std::time::Duration;

//Kafka dependencies
use kafka::producer::{Producer,Record,RequiredAcks};


//Local domains
use coinscrapper::coinmarketcap::CoinMarketEntry;


fn process_markets_info(producer : &mut Producer) {

    let c_market_link="https://api.coinmarketcap.com/v1/ticker";
    let mut resp = reqwest::get(c_market_link).unwrap();
    assert!(resp.status().is_success());
    let body = resp.text().unwrap();

    let markets_tick : Vec<CoinMarketEntry> = serde_json::from_str(body.as_str()).unwrap();
    let mut buf = String::with_capacity(2);

    markets_tick.iter().for_each(move |market|{
        let _ = write!(&mut buf,"{:?}",market);
        producer.send(&Record::from_value("coin.markets", buf.as_bytes())).unwrap();
        buf.clear();
    });

    println!("Processed markets");
}

fn main() {
    let js_str = r#"[{
    "id": "qora",
"name": "Qora",
"symbol": "QORA",
"rank": "1301",
"price_usd": "0.296948",
"price_btc": "0.00001774",
"24h_volume_usd": "380.547",
"market_cap_usd": null,
"available_supply": null,
"total_supply": "10000000000.0",
"max_supply": null,
"percent_change_1h": "-0.48",
"percent_change_24h": "-3.02",
"percent_change_7d": "13.96",
"last_updated": "1515325142"
}]
"#;

    let js_str_parsed = js_str.replace("24h_volume_usd", "_24h_volume_usd");
    let s: Vec<CoinMarketEntry> = serde_json::from_str(js_str_parsed.as_str()).unwrap();
    println!("Entry: {:?}", s);

    let c_market_link="https://api.coinmarketcap.com/v1/ticker";

    let mut resp = reqwest::get(c_market_link).unwrap();

    assert!(resp.status().is_success());

    //println!("{:?}",resp);

    let body = resp.text().unwrap();

    let markets_tick : Vec<CoinMarketEntry> = serde_json::from_str(body.as_str()).unwrap();

    println!("Markets: {}",markets_tick.len());

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
}