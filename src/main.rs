
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
extern crate serde_json;
extern crate term_painter;

use std::io;
use std::io::Write;
// JSON Parsing and Construction
// https://github.com/serde-rs/json
use serde_json::{Value, Error};

// HTTP library example taken from
// https://stackoverflow.com/questions/14154753/how-do-i-make-an-http-request-from-rust
// https://hyper.rs/guides/client/json/
use futures::{Future, Stream};
use hyper::{Client, Uri};
use hyper_tls::HttpsConnector;

use tokio_core::reactor::Core;

//https://lukaskalbertodt.github.io/term-painter/term_painter/
use term_painter::ToStyle;
use term_painter::Color::*;
use term_painter::Attr::*;

fn main() {
  let mut core = tokio_core::reactor::Core::new().unwrap();
    let handle = core.handle();
    let client = hyper::Client::configure()
        .connector(hyper_tls::HttpsConnector::new(4, &handle).unwrap())
        .build(&handle);

    let work = client.get("https://api.coinmarketcap.com/v1/ticker/?limit=10".parse().unwrap()).and_then(|res| {
        println!("Status: {}", res.status());
        println!("Headers:\n{}", res.headers());
        res.body().for_each(|chunk| {
            ::std::io::stdout().write_all(&chunk)
                .map(|_| ())
                .map_err(From::from)
        })
    });
    // request is a Future, futures are lazy, so must explicitly run
    core.run(work).unwrap();
    
    println!("Hello, world!");
    println!("{} or {} or {}",
        Red.paint("Red"),
        Bold.paint("Bold"),
        Red.bold().paint("Both!")
    );
}
