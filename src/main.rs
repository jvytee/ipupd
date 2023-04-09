mod config;
mod ipaddrs;
mod update;

use anyhow::Context;
use anyhow::Result;
use config::Config;
use getopts::Options;
use ipaddrs::IpAddrs;
use std::env;
use std::process;

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
        let request = update::create_request(&config, interface_ips);
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
