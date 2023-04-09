use ureq::Request;

use crate::config::Config;
use crate::ipaddrs::IpAddrs;

pub fn create_request(config: &Config, ip_addrs: IpAddrs) -> Request {
    let request = ureq::get(&config.url)
        .query(&config.query.ipv4, &ip_addrs.v4_string())
        .query(&config.query.ipv6, &ip_addrs.v6_string());

    if let Some(auth) = &config.basic_auth {
        request.set("Authorization", &auth.to_header())
    } else {
        request
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        config::{Config, Query},
        ipaddrs::IpAddrs,
    };

    use super::create_request;

    #[test]
    fn url() {
        let config = Config {
            interface: "eth0".to_string(),
            domain: "example.com".to_string(),
            url: "https://dyndns.example".to_string(),
            basic_auth: None,
            query: Query {
                ipv4: "ip".to_string(),
                ipv6: "ipv6".to_string(),
            },
        };
        let ip_addrs = IpAddrs {
            v4: None,
            v6: Some("::1".to_string()),
        };

        let request = create_request(&config, ip_addrs);
        assert_eq!(request.url(), "https://dyndns.example/?ip=0.0.0.0&ipv6=%3A%3A1");
    }
}
