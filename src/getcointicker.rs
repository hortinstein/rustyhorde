extern crate reqwest;
use std::io;
use std::io::Read;
/// coinprices
///
///
pub fn coinprices(limit: i32) ->  Result<String, reqwest::Error> {
    
    let mut uri: String = "https://api.coinmarketcap.com/v1/ticker/?limit=".to_owned();
    uri.push_str(&(limit.to_string()));
    println!("{}",uri);
    let mut resp = reqwest::get(uri)?;
    assert!(resp.status().is_success());

    let mut content = String::new();
    resp.read_to_string(&mut content);
    Ok(content)
}
