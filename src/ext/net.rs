pub fn is_ipv4_local(ip: &std::net::Ipv4Addr) -> bool {
    match ip.octets() {
        [10, _, _, _] => true,
        [172, 16..=32, _, _] => true,
        [192, 168, _, _] => true,
        _ => false,
    }
}
