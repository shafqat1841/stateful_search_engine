use crate::search_result::SearchResult;

#[derive(Debug)]
pub struct CacheEntry<'file_buffer> {
    pub values: Vec<SearchResult<'file_buffer>>,
    pub access_count: usize,
}

impl<'file_buffer> CacheEntry<'file_buffer> {
    pub fn new() -> Self {
        let values: Vec<SearchResult> = Vec::new();
        let access_count: usize = 0;
        CacheEntry {
            values,
            access_count,
        }
    }

    pub fn insert_result(&mut self, result: Vec<SearchResult<'file_buffer>>) {
        self.values = result;
    }

    pub fn increase_access_count(&mut self) {
        self.access_count += 1;
    }
}
