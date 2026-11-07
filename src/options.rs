//! Options model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Default,
    Address,
    Link,
    Route,
    Neigh,
    Tunnel,
    Json,
}

#[derive(Debug, Clone)]
pub struct IpOptions {
    pub mode: Mode,
    pub family: Option<String>,
}

impl Default for IpOptions {
    fn default() -> Self { Self { mode: Mode::Default, family: None } }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn opts() { assert_eq!(IpOptions::default().mode, Mode::Default); }
}
