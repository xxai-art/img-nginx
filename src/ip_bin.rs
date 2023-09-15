use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub fn ip_bin(ip_str: &str) -> Option<Vec<u8>> {
  match ip_str.parse::<IpAddr>() {
    Ok(IpAddr::V4(v4)) => Some(v4.octets().to_vec()),
    Ok(IpAddr::V6(v6)) => Some(v6.octets().to_vec()),
    Err(_) => None,
  }
}
