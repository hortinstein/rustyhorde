extern crate reqwest;
extern crate term_painter;

// JSON Parsing and Construction
// https://github.com/serde-rs/json
extern crate serde;
extern crate serde_json;

//local imports
extern crate getcointicker;


mod tools;

use std::str::FromStr;
use std::io;

//https://lukaskalbertodt.github.io/term-painter/term_painter/
use term_painter::ToStyle;
use term_painter::Color::*;
//use term_painter::Attr::*;
use getcointicker::coinprices;

#[macro_use]
extern crate serde_derive;

#[derive(Deserialize, Serialize, Debug)]
struct Coin {
    id: String,
    name: String,
    symbol: String,
    last_updated: String,
    price_usd: String,
    market_cap_usd: String,
    percent_change_1h: String,
    percent_change_24h: String,
    percent_change_7d: String
}

fn split_args(in_string: &str) -> (&str, f32) {
    let mut splitter = in_string.splitn(2, ':');
    let first = splitter.next().unwrap();
    let second = splitter.next().unwrap();
    let ret_second= f32::from_str(second).expect("should be an integer");
    (first, ret_second)
}

fn main() {


    //collects the arguments 
    let args: Vec<String> = std::env::args().collect();
    for arg in &args[1..]{
        println!("{:?}",split_args(&arg));
    }

    println!("rusty{}, ", Yellow.paint("Horde"),);
    
    //retrieve the prices for each coin
    let cp: String = match coinprices(25) {
        Result::Ok(val) => val,
        Result::Err(err) => format!("Unable to get coin prices: {}", err),
    };

    let v: Vec<Coin> = match serde_json::from_str(&cp) {
        Result::Ok(val) => val,
        Result::Err(err) => panic!("Unable to parse json: {}", err),
    };

    for item in &v {
        println!("{:?}",item.id);
    }

}
