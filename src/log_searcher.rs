use std::str::from_utf8;

use crate::{
    cache::Cache, cache_entries::CacheEntry, search_result::SearchResult,
    stateful_search_engine_errors::AllErros,
};

pub struct LogSearcher<'file_buffer> {
    bytes: &'file_buffer [u8],
    cache: Cache<'file_buffer>,
}

impl<'file_buffer> LogSearcher<'file_buffer> {
    pub fn new(bytes: &'file_buffer [u8], cache: Cache<'file_buffer>) -> Self {
        Self { bytes, cache }
    }

    pub fn get_search_result(
        &self,
        line_bytes: &'file_buffer [u8],
        query: &'file_buffer str,
    ) -> Option<SearchResult<'file_buffer>> {
        let line_str = from_utf8(line_bytes).ok()?;

        if line_str.contains(query) {
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

    pub fn search(&mut self, query: &'file_buffer str, limit: usize) -> Result<(), AllErros> {
        let bytes_split = self.bytes.split(|b| *b == b'\n');

        let filter_map_fun = |line_bytes| self.get_search_result(line_bytes, query);

        let result: Vec<SearchResult<'file_buffer>> = bytes_split
            // for development we are just checking 1000 lines only
            .take(limit)
            .filter_map(filter_map_fun)
            .collect();

        let mut cache_entry: CacheEntry<'file_buffer> = CacheEntry::new();
        cache_entry.insert_result(result);

        self.cache.entries.insert(query, cache_entry);

        Ok(())
    }

    pub fn get_all_query_entries(
        &self,
        query: &'file_buffer str,
    ) -> Option<&Vec<SearchResult<'file_buffer>>> {
        let entry: Option<&Vec<SearchResult<'file_buffer>>> = match self.cache.entries.get(query) {
            None => None,
            Some(val) => Some(&val.values),
        };

        entry
    }
}
