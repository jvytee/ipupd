mod ip_addrs;

use getopts::Options;
use ip_addrs::IpAddrs;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut opts = Options::new();
    opts.optflag("h", "help", "show help");

    let args: Vec<String> = env::args().collect();
    let matches = opts.parse(&args).expect("Could not parse arguments");

    if matches.free.len() < 3 || matches.opt_present("h") {
        let usage = format!("Usage: {} INTERFACE DOMAIN", args[0]);
        println!("{}", opts.usage(&usage));
        return Ok(());
    }

    let interface = &matches.free[1];
    let interface_ips = IpAddrs::from_interface(interface).expect(&format!("Could not find interface {}", interface));
    println!("Interface: {}\n\
              IPv4: {}\n\
              IPv6: {}\n", interface, interface_ips.v4.unwrap_or("None".to_string()), interface_ips.v6.unwrap_or("None".to_string()));


    let domain = &matches.free[2];
    let domain_ips = IpAddrs::from_domain(domain).expect(&format!("Could not resolve {}", domain));
    println!("Domain: {}\n\
              IPv4: {}\n\
              IPv6: {}", domain, domain_ips.v4.unwrap_or("None".to_string()), domain_ips.v6.unwrap_or("None".to_string()));

    Ok(())
}
