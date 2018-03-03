extern crate term_painter;
extern crate reqwest;


extern crate serde_json;
// JSON Parsing and Construction
// https://github.com/serde-rs/json
use serde_json::{Value};

extern crate getcointicker;

//use std::io;

//https://lukaskalbertodt.github.io/term-painter/term_painter/
use term_painter::ToStyle;
use term_painter::Color::*;
//use term_painter::Attr::*;
use getcointicker::coinprices;

fn main() {
    println!("rusty{}, ",
        Yellow.paint("Horde"),
    );
    let cp: String = match coinprices(25) {
        Result::Ok(val) => {val},
        Result::Err(err) => {format!("Unable to get coin prices: {}",err)}
    };

    let v: Vec<Value> = match serde_json::from_str(&cp){
        Result::Ok(val) => {val},
        Result::Err(err) => {panic!("Unable to parse json: {}",err)}
    };

    for item in &v {
        println!("{:?}\n", item);
    }
        
    println!("{}",v[0]["market_cap_usd"]);  
}
