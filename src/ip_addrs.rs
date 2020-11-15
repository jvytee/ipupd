use pnet::datalink::interfaces;
use pnet::ipnetwork::IpNetwork;
use std::net::{
    SocketAddr,
    ToSocketAddrs
};

#[derive(Debug)]
pub struct IpAddrs {
    pub v4: Option<String>,
    pub v6: Option<String>,
}

impl IpAddrs {
    pub fn new() -> IpAddrs {
        IpAddrs {
            v4: None,
            v6: None,
        }
    }

    pub fn from_domain(domain: &str) -> std::io::Result<IpAddrs> {
        let socket_addrs = format!("{}:443", domain).to_socket_addrs()?;
        let mut ip_addr = IpAddrs::new();

        for socket_addr in socket_addrs {
            match socket_addr {
                SocketAddr::V4(v4_addr) => ip_addr.v4 = Some(v4_addr.ip().to_string()),
                SocketAddr::V6(v6_addr) => ip_addr.v6 = Some(v6_addr.ip().to_string())
            }
        }

        Ok(ip_addr)
    }

    pub fn from_interface(name: &str) -> Option<IpAddrs> {
        let ip_networks: Option<Vec<IpNetwork>> = interfaces().iter()
            .filter(|interface| interface.name == name)
            .map(|interface| interface.ips.clone())
            .next();

        return if let Some(ip_networks) = ip_networks {
            let mut ip_addr = IpAddrs::new();

            for ip_network in ip_networks {
                match ip_network {
                    IpNetwork::V4(v4_network) => ip_addr.v4 = Some(v4_network.ip().to_string()),
                    IpNetwork::V6(v6_network) => ip_addr.v6 = Some(v6_network.ip().to_string())
                }
            }

            Some(ip_addr)
        } else {
            None
        }
    }
}
