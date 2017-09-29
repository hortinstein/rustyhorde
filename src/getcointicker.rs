extern crate reqwest;

use std::io::Read;
use reqwest::get;
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
pub fn coinprices(limit: i32) ->  Result<String, reqwest::Error> {
    /// 
    let mut uri: String = "https://api.coinmarketcap.com/v1/ticker/?limit=".to_owned();
    uri.push_str(&(limit.to_string()));
  
    let mut resp = reqwest::get(&uri)?;
    assert!(resp.status().is_success());

    let mut content = String::new();
    resp.read_to_string(&mut content);
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
pub fn coinprice_id(id: String) ->  Result<String, reqwest::Error> {
    /// 
    let mut uri: String = "https://api.coinmarketcap.com/v1/ticker/".to_owned();
    uri.push_str(&id);
  
    let mut resp = reqwest::get(&uri)?;
    assert!(resp.status().is_success());

    let mut content = String::new();
    resp.read_to_string(&mut content);
    Ok(content)
}



////////////////////////////////////////
//Test code
///////////////////////////////////////
extern crate serde_json;
// JSON Parsing and Construction
// https://github.com/serde-rs/json
use serde_json::from_str;

#[test]
fn test_pull_id() {
    let cp = match coinprice_id("bitcoin".to_owned()) {
    Result::Ok(val) => {val},
    Result::Err(err) => {format!("OH NO")}    
    };
    let v: Value = serde_json::from_str(&cp).unwrap();
    println!("{}",v);
    assert!(v[0]["symbol"] == "BTC");
}

#[test]
fn test_pull_count() {
    let cp = match coinprices(4) {
    Result::Ok(val) => {val},
    Result::Err(err) => {format!("OH NO")}    
    };
    let v: Value = serde_json::from_str(&cp).unwrap();
    //https://docs.serde.rs/serde_json/value/enum.Value.html
    assert!(v.as_array().unwrap().len() == 4);
}