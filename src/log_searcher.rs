use std::str::from_utf8;

use crate::{search_result::SearchResult, stateful_search_engine_errors::AllErros};

pub struct LogSearcher<'a> {
    bytes: &'a [u8],
}

impl<'a> LogSearcher<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }

    pub fn get_search_result(&self, line_bytes: &'a [u8], query: &str) -> Option<SearchResult<'a>> {
        let line_str = from_utf8(line_bytes).ok()?;

        if line_str.contains(query) {
            let end_idx = line_str.find(" - - ")?;

            let ip_address = line_str.get(..end_idx)?;

            Some(SearchResult {
                line: line_str,
                ip_address,
            })
        } else {
            None
        }
    }

    pub fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult<'a>>, AllErros> {
        let bytes_split = self.bytes.split(|b| *b == b'\n');

        let filter_map_fun = |line_bytes| self.get_search_result(line_bytes, query);

        let result: Vec<SearchResult> = bytes_split
            // for development we are just checking 1000 lines only
            .take(limit)
            .filter_map(filter_map_fun)
            .collect();

        Ok(result)
    }
}
