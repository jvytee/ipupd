use getopts::Options;
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

#[derive(Debug)]
struct IpAddrs {
    v4: Option<String>,
    v6: Option<String>,
}

impl IpAddrs {
    fn new() -> IpAddrs {
        IpAddrs {
            v4: None,
            v6: None,
        }
    }

    fn from_domain(domain: &str) -> std::io::Result<IpAddrs> {
        let socket_addrs = format!("{}:443", domain).to_socket_addrs()?;
        let mut ip_addr = IpAddrs::new();

        for socket_addr in socket_addrs {
            match socket_addr {
                V4(v4_addr) => ip_addr.v4 = Some(v4_addr.ip().to_string()),
                V6(v6_addr) => ip_addr.v6 = Some(v6_addr.ip().to_string())
            }
        }

        Ok(ip_addr)
    }
}
