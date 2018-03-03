extern crate reqwest;
extern crate term_painter;

extern crate serde_json;
// JSON Parsing and Construction
// https://github.com/serde-rs/json
use serde_json::Value;

extern crate getcointicker;

//use std::io;

//https://lukaskalbertodt.github.io/term-painter/term_painter/
use term_painter::ToStyle;
use term_painter::Color::*;
//use term_painter::Attr::*;
use getcointicker::coinprices;

struct Coin {
    id: String,
    name: String,
    symbol: String,
    last_updated: u64,
    price_usd: f64,
    market_cap_usd: f64,
    percent_change_1h: f32,
    percent_change_24h: f32,
    percent_change_7d: f32
}

fn main() {
    println!("rusty{}, ", Yellow.paint("Horde"),);
    let cp: String = match coinprices(25) {
        Result::Ok(val) => val,
        Result::Err(err) => format!("Unable to get coin prices: {}", err),
    };

    let v: Vec<Value> = match serde_json::from_str(&cp) {
        Result::Ok(val) => val,
        Result::Err(err) => panic!("Unable to parse json: {}", err),
    };

    for item in &v {

        println!("{}, {}, {}, {}, {}, {}, {}, {}, {}", 
                    item["id"], 
                    item["name"], 
                    item["symbol"], 
                    item["last_updated"], 
                    item["price_usd"],
                    item["market_cap_usd"],
                    item["percent_change_1h"],
                    item["percent_change_24h"],
                    item["percent_change_7d"] 
                    );
        //println!("{:?}",item);
    }

    println!("{}", v[0]["market_cap_usd"]);
}
