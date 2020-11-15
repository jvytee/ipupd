mod ip_addrs;

use getopts::Options;
use ip_addrs::IpAddrs;
use std::env;
use std::error::Error;
use std::net::{
    SocketAddr::{V4, V6},
    ToSocketAddrs
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut opts = Options::new();
    opts.optflag("h", "help", "show help");

    let args: Vec<String> = env::args().collect();
    let matches = opts.parse(&args).expect("Could not parse arguments");

    if matches.free.len() < 2 || matches.opt_present("h") {
        let usage = format!("Usage: {} DOMAIN", args[0]);
        println!("{}", opts.usage(&usage));
        return Ok(());
    }

    let domain = &matches.free[1];
    let ip_addrs = IpAddrs::from_domain(domain).expect(&format!("Could not resolve {}", domain));
    println!("Domain: {}\n\
              IPv4: {}\n\
              IPv6: {}", domain, ip_addrs.v4.unwrap_or("None".to_string()), ip_addrs.v6.unwrap_or("None".to_string()));

    Ok(())
}
