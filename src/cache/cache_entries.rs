use crate::log_searcher::SearchResult;


#[derive(Debug)]
pub struct CacheEntry<'file_buffer> {
    pub values: Vec<SearchResult<'file_buffer>>,
    pub node_index: usize,
}

impl<'file_buffer> CacheEntry<'file_buffer> {
    pub fn new(values: Vec<SearchResult<'file_buffer>>, node_index: usize) -> Self {
        CacheEntry { values, node_index }
    }
}
