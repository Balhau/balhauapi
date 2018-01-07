/**

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
*/


#[derive(Debug,Serialize, Deserialize)]
pub struct CoinMarketEntry {
    id: String,
    name: String,
    symbol: String,
    rank: String,
    price_usd: String,
    price_btc: String,
    _24h_volume_usd: Option<String>,
    market_cap_usd: Option<String>,
    available_supply: Option<String>,
    total_supply: String,
    max_suplly: Option<String>,
    percent_change_1h: String,
    percent_change_24h: String,
    percent_change_7d: String,
    last_updated: String
}



