
extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate serde_json;
extern crate term_painter;

use std::io;

// JSON Parsing and Construction
// https://github.com/serde-rs/json
use serde_json::{Value, Error};

// HTTP library example taken from
// https://stackoverflow.com/questions/14154753/how-do-i-make-an-http-request-from-rust
// https://hyper.rs/guides/client/json/
use futures::{Future, Stream};
use hyper::{Client, Uri};
use tokio_core::reactor::Core;

//https://lukaskalbertodt.github.io/term-painter/term_painter/
use term_painter::ToStyle;
use term_painter::Color::*;
use term_painter::Attr::*;




fn main() {

    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());
    let url : Uri = "https://api.coinmarketcap.com/v1/ticker/".parse().unwrap();

    let request = client.get(url).and_then(|res| {
        println!("Response: {}",res.status());
        res.body().concat2().and_then( move |body| {
            let v: Value = serde_json::from_slice(&body).map_err(|e| {
                io::Error::new(
                    io::ErrorKind::Other,
                    e
                )  
            }).unwrap();
            println!("current Bitcoin price is {:?}", v);
            Ok(())
        })
    });

    // request is a Future, futures are lazy, so must explicitly run
    
    core.run(request).unwrap();
    
    
    println!("Hello, world!");
    println!("{} or {} or {}",
        Red.paint("Red"),
        Bold.paint("Bold"),
        Red.bold().paint("Both!")
    );
}
