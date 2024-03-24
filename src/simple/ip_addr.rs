use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[allow(dead_code)]
pub fn using_ipaddr() {
    let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let localhost_v6 = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
    println!("std: ip_address_v4: {}", localhost_v4);
    println!("std: ip_address_v6: {}", localhost_v6);
}
