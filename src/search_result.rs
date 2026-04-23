use std::fmt::Display;

#[derive(Debug)]
pub struct SearchResult<'a> {
    pub line: &'a str,       // Reference to the full line in the NASA log
    pub ip_address: &'a str, // Reference to just the IP part of that line
}

impl<'a> Display for SearchResult<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ip_address: {} \n line: {}", self.ip_address, self.line)
    }
}
