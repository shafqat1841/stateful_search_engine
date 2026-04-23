mod file_buffer;
mod log_searcher;
mod stateful_search_engine_errors;
mod search_result;

use std::path::PathBuf;

use crate::{
    file_buffer::FileBuffer, log_searcher::LogSearcher, search_result::SearchResult, stateful_search_engine_errors::AllErros
};

pub fn run() -> Result<(), AllErros> {
    let path: PathBuf = PathBuf::from("./log_files/access.log");
    let query: String= "302".to_string();
    let limit: usize = 1000;

    let file_buffer = FileBuffer::new(&path)?;

    let bytes: &[u8] = file_buffer.get_bytes();

    let log_searcher: LogSearcher = LogSearcher::new(bytes);

    let result: Vec<SearchResult<'_>> = log_searcher.search(&query, limit)?;

    for search_result in result {
        println!("{}", search_result)
    }

    Ok(())
}
