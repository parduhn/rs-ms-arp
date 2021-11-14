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

```json
{
  "mac_addr": "98:01:a7",
  "vendor_name": "Apple Inc.",
  "ip4": "102.12.123.1"
}
```
## Release
Deploy service witn `deploy.sh`. This copies the batch file `start_all.sh` to the directory /timeover/serivces. Use the script to start all service in this folder. the script will create a screen in the name of the service and start each service in this screen. 

This is located here, because it is the first service.

## Based on
A simple web API to do an [arp](https://en.wikipedia.org/wiki/Address_Resolution_Protocol) scan on the local network and get human-readable results of vendor hardware on the local wifi network.

## Future
Nmap could help to see OS and Ports: https://github.com/sommd/rustmap