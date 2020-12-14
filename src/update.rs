use crate::config::{Auth, Query};
use crate::ipaddrs::IpAddrs;
use std::error::Error;
use std::fmt::{Debug, Display};

pub fn update(url: &str, query: &Query, ip_addrs: IpAddrs, basic_auth: Option<Auth>) -> Result<String, HttpError> {
    let mut request = if let Some(auth) = basic_auth {
        ureq::get(url)
            .query(&query.ipv4, &ip_addrs.v4_string())
            .query(&query.ipv6, &ip_addrs.v6_string())
            .auth(&auth.username, &auth.password)
            .clone()
    } else {
        ureq::get(url)
            .query(&query.ipv4, &ip_addrs.v4_string())
            .query(&query.ipv6, &ip_addrs.v6_string())
            .clone()
    };

    let response = request.call();
    return if let Some(error) = response.synthetic_error() {
        Err(HttpError {
            status_code: error.status(),
        })
    } else {
        Ok(response.into_string().unwrap_or(String::new()))
    };
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
