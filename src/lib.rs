mod cache;
mod cache_entries;
mod file_buffer;
mod log_searcher;
mod search_result;
mod stateful_search_engine_errors;

use std::path::PathBuf;

use crate::{
    cache::Cache, file_buffer::FileBuffer, log_searcher::LogSearcher, search_result::SearchResult,
    stateful_search_engine_errors::AllErros,
};

fn search_logic<'file_buffer>(
    bytes: &'file_buffer [u8],
    query: &'file_buffer str,
    limit: usize,
) -> Result<(), AllErros> {
    let cache: Cache<'file_buffer> = Cache::new();

    let mut log_searcher: LogSearcher<'file_buffer> = LogSearcher::new(bytes, cache);

    let _ = log_searcher.search(query, limit)?;
    let result: Option<&Vec<SearchResult<'file_buffer>>> =
        log_searcher.get_all_query_entries(query);

    match result {
        None => {
            println!("No result found for this query: {}", query);
        }
        Some(val) => {
            if val.is_empty() {
                println!("No result found for this query: {}", query);
            } else {
                for search_result in val {
                    let print_value: &SearchResult<'file_buffer> = search_result;
                    println!("{}", print_value)
                }
            }
        }
    }

    Ok(())
}

pub fn run() -> Result<(), AllErros> {
    let path: PathBuf = PathBuf::from("./log_files/access.log");
    // let query: &str = "302";
    let query: &str = "sdfasdfsa";
    let limit: usize = 1000;

    let file_buffer = FileBuffer::new(&path)?;

    let bytes: &[u8] = file_buffer.get_bytes();

    search_logic(bytes, query, limit)
}
