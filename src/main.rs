mod config;
mod ipaddrs;
mod update;

use anyhow::Result;
use config::Config;
use getopts::Options;
use ipaddrs::IpAddrs;
use std::env;
use std::process;


fn main() {
    if let Err(error) = try_main() {
        eprintln!("{}", error);
        process::exit(2);
    }
}

fn try_main() -> Result<()> {
    let mut opts = Options::new();
    opts.optflag("h", "help", "show help and exit");
    opts.optopt("c", "config", "configuration file", "FILE");

    let args: Vec<String> = env::args().collect();
    let matches = opts.parse(&args)?;

    if matches.opt_present("h") {
        let usage = format!("Usage: {} [OPTIONS]", args[0]);
        println!("{}", opts.usage(&usage));
        return Ok(());
    }

    let config_file = matches.opt_str("c").unwrap_or("/etc/ipupd/config.toml".to_string());
    match Config::from_file(&config_file) {
        Ok(config) => {
            let interface = &config.interface;
            let interface_ips = IpAddrs::from_interface(interface)
                .expect(&format!("Could not inspect {}", interface));

            let domain = &config.domain;
            let domain_ips = IpAddrs::from_domain(domain)
                .expect(&format!("Could not resolve {}", domain));

            if interface_ips != domain_ips {
                let response = update::update(&config.url, &config.query, interface_ips, config.basic_auth)?;
                println!("{}", response);
            }
        }
        Err(error) => {
            println!("Could not parse config file {}: {}", &config_file, error);
        }
    };

    Ok(())
}
