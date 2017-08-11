extern crate reqwest;
use std::io;
use std::io::Read;
/// coinprices
///
///
pub fn coinprices(limit: i32) ->  Result<String, reqwest::Error> {
    let uri = "https://api.coinmarketcap.com/v1/ticker/?limit=10";

    let mut resp = reqwest::get(uri)?;
    assert!(resp.status().is_success());

    let mut content = String::new();
    resp.read_to_string(&mut content);
    Ok(content)
}
