mod config;
mod ipaddrs;
mod update;

use config::Config;
use getopts::Options;
use ipaddrs::IpAddrs;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut opts = Options::new();
    opts.optflag("h", "help", "show help and exit");
    opts.optopt("c", "config", "configuration file", "FILE");

    let args: Vec<String> = env::args().collect();
    let matches = opts.parse(&args).expect("Could not parse arguments");

    if matches.opt_present("h") {
        let usage = format!("Usage: {}", args[0]);
        println!("{}", opts.usage(&usage));
        return Ok(());
    }

    let config_file = matches.opt_str("c").unwrap_or("config.toml".to_string());
    match Config::from_file(&config_file) {
        Ok(config) => {
            let interface = &config.interface;
            let interface_ips = IpAddrs::from_interface(interface)
                .expect(&format!("Could not inspect {}", interface));

            let domain = &config.domain;
            let domain_ips = IpAddrs::from_domain(domain)
                .expect(&format!("Could not resolve {}", domain));

            let res = update::update(&config.url, &config.query, interface_ips, config.basic_auth);
            println!("{:?}", res);
        }
        Err(error) => {
            println!("Could not parse config file {}: {}", &config_file, error);
        }
    };

    Ok(())
}
