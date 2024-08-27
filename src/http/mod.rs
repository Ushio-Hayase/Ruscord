use std::net::IpAddr;

use reqwest;

struct HTTPClient {
    client: reqwest::Client,
    ip: IpAddr,
    url: String,
    target_ip: IpAddr
}