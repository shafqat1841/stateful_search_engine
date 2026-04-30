mod search_result;

use std::str::from_utf8;

pub use crate::log_searcher::search_result::SearchResult;
use crate::stateful_search_engine_errors::AllErros;

pub struct LogSearcher<'file_buffer> {
    bytes: &'file_buffer [u8],
}

impl<'file_buffer> LogSearcher<'file_buffer> {
    pub fn new(bytes: &'file_buffer [u8]) -> Self {
        Self { bytes }
    }

    pub fn get_search_result(
        &self,
        line_bytes: &'file_buffer [u8],
        query: &str,
    ) -> Option<SearchResult<'file_buffer>> {
        let line_str = from_utf8(line_bytes).ok()?;

        let contain_query = line_str.contains(query);

        if contain_query {
            let end_idx = line_str.find(" - - ")?;

            let ip_address = line_str.get(..end_idx)?;

            let data: SearchResult<'file_buffer> = SearchResult {
                line: line_str,
                ip_address,
            };

            Some(data)
        } else {
            None
        }
    }

    pub fn search(
        &mut self,
        query: &str,
        limit: Option<usize>,
    ) -> Result<Vec<SearchResult<'file_buffer>>, AllErros> {
        let bytes_split = self.bytes.split(|b| *b == b'\n');

        let filter_map_fun = |line_bytes| self.get_search_result(line_bytes, query);

        if let Some(val) = limit {
            let res_take = bytes_split.take(val);
            let result: Vec<SearchResult<'file_buffer>> =
                res_take.filter_map(filter_map_fun).collect();

            Ok(result)
        } else {
            let res_take = bytes_split;
            let result: Vec<SearchResult<'file_buffer>> =
                res_take.filter_map(filter_map_fun).collect();

            Ok(result)
        }
    }
}
