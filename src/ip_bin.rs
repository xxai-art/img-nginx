use std::net::IpAddr;

pub fn ip_bin(ip_str: &str) -> Option<Vec<u8>> {
  match ip_str.parse::<IpAddr>() {
    Ok(IpAddr::V4(v4)) => Some(v4.octets().to_vec()),
    Ok(IpAddr::V6(v6)) => Some(v6.octets().to_vec()),
    Err(_) => None,
  }
}

// pub fn bin_ip(bytes: &[u8]) -> Option<String> {
//   match bytes.len() {
//     4 => {
//       let addr = Ipv4Addr::new(bytes[0], bytes[1], bytes[2], bytes[3]);
//       Some(addr.to_string())
//     }
//     16 => {
//       let addr = Ipv6Addr::new(
//         (bytes[0] as u16) << 8 | bytes[1] as u16,
//         (bytes[2] as u16) << 8 | bytes[3] as u16,
//         (bytes[4] as u16) << 8 | bytes[5] as u16,
//         (bytes[6] as u16) << 8 | bytes[7] as u16,
//         (bytes[8] as u16) << 8 | bytes[9] as u16,
//         (bytes[10] as u16) << 8 | bytes[11] as u16,
//         (bytes[12] as u16) << 8 | bytes[13] as u16,
//         (bytes[14] as u16) << 8 | bytes[15] as u16,
//       );
//       Some(addr.to_string())
//     }
//     _ => None,
//   }
// }
