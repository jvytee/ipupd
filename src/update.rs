use crate::config::{Auth, Query};
use crate::ipaddrs::IpAddrs;
use std::error::Error;
use std::fmt::{Debug, Display};

pub fn update(url: &str, query: &Query, ip_addrs: IpAddrs, basic_auth: Option<Auth>) -> Result<String, ureq::Error> {
    let request = if let Some(auth) = basic_auth {
        ureq::get(url)
            .query(&query.ipv4, &ip_addrs.v4_string())
            .query(&query.ipv6, &ip_addrs.v6_string())
            .set("Authorization", &auth.to_header())
            .clone()
    } else {
        ureq::get(url)
            .query(&query.ipv4, &ip_addrs.v4_string())
            .query(&query.ipv6, &ip_addrs.v6_string())
            .clone()
    };

    match request.call() {
        Ok(response) => Ok(response.into_string().unwrap_or(String::new())),
        Err(error) => Err(error)
    }
}

#[derive(Debug)]
pub struct HttpError {
    status_code: u16,
}

impl Display for HttpError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(formatter, "HTTP error: {}", self.status_code)
    }
}

impl Error for HttpError {}
