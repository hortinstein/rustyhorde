extern crate reqwest;

use std::io::Read;
//use reqwest::get;

/// returns a list of coin prices from coinmarketcap
///
/// # Arguments
///
/// * `limit` - the number of results to return
///
/// # Example
///
/// ```
/// use getcointicker::coinprices;
///
/// let cp = match coinprices(4) {
///     Result::Ok(val) => {val},
///     Result::Err(err) => {format!("OH NO")}
/// };
/// println!("{}",cp);
/// ```
pub fn coinprices(limit: i32) -> Result<String, reqwest::Error> {
    let mut uri: String = "https://api.coinmarketcap.com/v1/ticker/?limit=".to_owned();
    uri.push_str(&(limit.to_string()));

    let mut resp = reqwest::get(&uri)?;
    assert!(resp.status().is_success());

    let mut content = String::new();
    resp.read_to_string(&mut content)
        .expect("could not read prices");;
    Ok(content)
}

/// returns a coin's price from coinmarketcap
///
/// # Arguments
///
/// * `id` - the number of results to return
///
/// # Example
///
/// ```
/// use getcointicker::coinprice_id;
///
/// let cp = match coinprice_id("bitcoin".to_owned()) {
///     Result::Ok(val) => {val},
///     Result::Err(err) => {format!("OH NO")}
/// };
/// println!("{}",cp);
/// ```
pub fn coinprice_id(id: String) -> Result<String, reqwest::Error> {
    println!("retrieving price information for: {}",&id);
    let mut uri: String = "https://api.coinmarketcap.com/v1/ticker/".to_owned();
    uri.push_str(&id);

    let mut resp = reqwest::get(&uri)?;
    assert!(resp.status().is_success());

    let mut content = String::new();
    resp.read_to_string(&mut content)
        .expect("could not read prices");
    Ok(content)
}

////////////////////////////////////////
//Test code
///////////////////////////////////////
extern crate serde_json;
// JSON Parsing and Construction
// https://github.com/serde-rs/json

//unused imports are allowed here because this is just for test code
#[allow(unused_imports)]
use serde_json::from_str;

#[allow(unused_imports)]
use serde_json::Value;
#[test]
fn test_pull_id() {
    let cp = match coinprice_id("bitcoin".to_owned()) {
        Result::Ok(val) => val,
        Result::Err(err) => format!("coinprice_id failed: {}", err),
    };
    let v: Value = serde_json::from_str(&cp).unwrap();
    println!("{}", v);
    assert!(v[0]["symbol"] == "BTC");
}

#[test]
fn test_pull_count() {
    let cp = match coinprices(4) {
        Result::Ok(val) => val,
        Result::Err(err) => format!("coinprices failed: {}", err),
    };
    let v: Value = serde_json::from_str(&cp).unwrap();
    assert!(v.as_array().unwrap().len() == 4);
}
