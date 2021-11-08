# ARP Microservice
Service to recive devices via ARP in network and sends it to MessageQueue

## What it does
* API sends ARP request to each IP address on local network and receives MAC addresses
* API matches each MAC address against cached list in app state
* API calls macvendors.com for each MAC that is found that is not in app state
* Results from macvendors are added to cache, so that macvendors is not bothered anymore
* Sends chached and unchaced MAC and IPs to MessageQueue

## Run
Run wiht `sudo cargo run` 

## Result
Pushes messages to MessageQueue System as Json, one every second
GET /arp
```json
{
  "mac_addr": "98:01:a7",
  "vendor_name": "Apple Inc.",
  "ip4": "102.12.123.1"
}
```

## Based on
A simple web API to do an [arp](https://en.wikipedia.org/wiki/Address_Resolution_Protocol) scan on the local network and get human-readable results of vendor hardware on the local wifi network.

