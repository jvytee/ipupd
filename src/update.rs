use ureq::Request;

use crate::config::Config;
use crate::ipaddrs::IpAddrs;

pub fn create_request(
    config: &Config,
    ip_addrs: IpAddrs,
) -> Request {
    let request = ureq::get(&config.url)
        .query(&config.query.ipv4, &ip_addrs.v4_string())
        .query(&config.query.ipv6, &ip_addrs.v6_string());

    if let Some(auth) = &config.basic_auth {
        request.set("Authorization", &auth.to_header())
    } else {
        request
    }
}
