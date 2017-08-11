extern crate term_painter;
extern crate reqwest;

mod coinparse;
mod getcointicker;

use std::io;

//https://lukaskalbertodt.github.io/term-painter/term_painter/
use term_painter::ToStyle;
use term_painter::Color::*;
use term_painter::Attr::*;
use getcointicker::coinprices;
fn main() {

    println!("Hello, world!");
    println!("{} or {} or {}",
        Red.paint("Red"),
        Bold.paint("Bold"),
        Red.bold().paint("Both!")
    );
    println!("{}",coinprices(4));
}
