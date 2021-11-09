// #[test]
// fn it_gets_vendor_name() {
//     let vendor_url = "https://api.macvendors.com".to_string();
//     let result = vendor_request(&vendor_url, &"4C:32:75".to_string());
//     assert_eq!("Apple, Inc.".to_string(), result.unwrap());
// }

//for calling macvendor api
//GET https://api.macvendors.com/98-01-A7 plaintext response

use reqwest::{Client, Error as RWError};

//dashes are not needed for calling macvendors api, capitalization doesn't matter
pub fn vendor_request(client: &Client, url: &str, mac_addr: &str) -> Result<String, RWError> {
    println!("Get Vendor");
    let resp = client.get(&format!("{}/{}", url, mac_addr)).send();
    // println!("Response {:?}", resp.status());
    Ok(resp?.text()?)
}
