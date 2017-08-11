extern crate term_painter;
extern crate reqwest;


extern crate serde_json;
// JSON Parsing and Construction
// https://github.com/serde-rs/json
use serde_json::{Value, Error};

mod getcointicker;

use std::io;

//https://lukaskalbertodt.github.io/term-painter/term_painter/
use term_painter::ToStyle;
use term_painter::Color::*;
use term_painter::Attr::*;
use getcointicker::coinprices;

fn main() {
    println!("rusty{}, ",
        Yellow.paint("Horde"),
    );
    let cp = match coinprices(4) {
        Result::Ok(val) => {val},
        Result::Err(err) => {format!("OH NO")}
    };
    println!("{}",cp);  
//     let v: Value = serde_json::from_str(&cp);
//     println!("{}",v[0]["market_cap_usd"]);  
}
