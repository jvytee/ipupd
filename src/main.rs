mod config;
mod ipaddrs;

use anyhow::{Context, Result};
use config::Config;
use getopts::Options;
use ipaddrs::IpAddrs;
use std::env;
use std::process;
use ureq::Request;

const DEFAULT_CONFIG: &str = "/etc/ipupd/config.toml";

fn main() {
    if let Err(error) = try_main() {
        eprintln!("{}", error);
        process::exit(2);
    }
}

fn try_main() -> Result<()> {
    let mut opts = Options::new();
    opts.optflag("h", "help", "Show help and exit");
    opts.optopt(
        "c",
        "config",
        &format!("Configuration file (default: {})", DEFAULT_CONFIG),
        "FILE",
    );

    let args: Vec<String> = env::args().collect();
    let matches = opts.parse(&args)?;

    if matches.opt_present("h") {
        let usage = format!("Usage: {} [OPTIONS]", args[0]);
        println!("{}", opts.usage(&usage));
        return Ok(());
    }

    let config_file = matches.opt_str("c").unwrap_or(DEFAULT_CONFIG.to_string());
    let config = Config::from_file(&config_file)
        .with_context(|| format!("Could not parse config file {}", &config_file))?;

    let interface = &config.interface;
    let interface_ips = IpAddrs::from_interface(interface)
        .with_context(|| format!("Could not inspect {}", interface))?;

    let domain = &config.domain;
    let domain_ips =
        IpAddrs::from_domain(domain).with_context(|| format!("Could not resolve {}", domain))?;

    if interface_ips != domain_ips {
        let request = create_request(&config, &interface_ips);
        let response = request.call().context("Could not perform GET request")?;
        println!(
            "{}",
            response
                .into_string()
                .context("Could not read response body")?
        );
    }

    Ok(())
}

fn create_request(config: &Config, ip_addrs: &IpAddrs) -> Request {
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

    fn test_url_query(ip_addrs: &IpAddrs, url: &str) {
        let config = Config {
            interface: "eth0".to_string(),
            domain: "foobar.example".to_string(),
            url: "https://dyndns.example".to_string(),
            basic_auth: None,
            query: Query {
                ipv4: "foo".to_string(),
                ipv6: "bar".to_string(),
            },
        };

        let request = create_request(&config, ip_addrs);
        assert_eq!(request.url(), url);
    }

    #[test]
    fn query_empty() {
        let ip_addrs = IpAddrs { v4: None, v6: None };
        let url = "https://dyndns.example/?foo=0.0.0.0&bar=%3A%3A";
        test_url_query(&ip_addrs, url);
    }
}
