#[cfg(test)]
mod tests {
    use ms_arp_lib::arp::macvendor::vendor_request;
    use reqwest::Client;
    #[test]
    fn it_gets_vendor_name() {
        let vendor_url = "https://api.macvendors.com".to_string();
        let client = Client::new();
        // let result =
        //     ms_arp_lib::api::macvendor::vendor_request(&vendor_url, &"4C:32:75".to_string());
        let result = vendor_request(&client, &vendor_url, &"4C:32:75".to_string());
        assert_eq!("Apple, Inc.".to_string(), result.unwrap());
    }
}
