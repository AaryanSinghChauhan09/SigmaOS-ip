//! Ip model — show/manipulate IP addresses, routes, links.

#[derive(Debug, Clone)]
pub struct AddrInfo {
    pub iface: String,
    pub family: String,
    pub address: String,
    pub prefixlen: u8,
    pub scope: String,
}

#[derive(Debug, Clone)]
pub struct RouteInfo {
    pub dest: String,
    pub gateway: String,
    pub dev: String,
    pub metric: u32,
}

#[derive(Debug, Clone)]
pub struct Ip {
    pub json: bool,
    pub details: bool,
}

impl Default for Ip {
    fn default() -> Self { Self { json: false, details: false } }
}

impl Ip {
    pub fn new() -> Self { Self::default() }

    pub fn parse_addr(&self, spec: &str) -> Option<(String, u8)> {
        spec.split_once('/')
            .and_then(|(a, p)| p.parse().ok().map(|n| (a.to_string(), n)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_addr() {
        let i = Ip::new();
        assert_eq!(i.parse_addr("192.168.1.1/24").unwrap(), ("192.168.1.1".into(), 24));
    }
}
