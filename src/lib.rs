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

fn show_result<'file_buffer>(query: &'file_buffer str,result: Option<&Vec<SearchResult<'file_buffer>>>) {
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
}

fn search_logic<'file_buffer>(
    bytes: &'file_buffer [u8],
    query: &'file_buffer str,
    limit: usize,
) -> Result<(), AllErros> {
    let mut cache: Cache<'file_buffer> = Cache::new();

    if cache.check_query(query) {
        let result: Option<&Vec<SearchResult<'file_buffer>>> = cache.get_query_value(query);
        show_result(query, result);
        return Ok(());
    }

    let mut log_searcher: LogSearcher<'file_buffer> = LogSearcher::new(bytes);

    let query_result: Vec<SearchResult<'file_buffer>> = log_searcher.search(query, limit)?;

    cache.insert_query_result(query, query_result);

    let result: Option<&Vec<SearchResult<'file_buffer>>> = cache.get_query_value(query);

    show_result(query, result);

    Ok(())
}

pub fn run() -> Result<(), AllErros> {
    let path: PathBuf = PathBuf::from("./log_files/access.log");
    let limit: usize = 1000;

    let query: &str = "302";
    // let query: &str = "sdfasdfsa";

    let file_buffer = FileBuffer::new(&path)?;

    let bytes: &[u8] = file_buffer.get_bytes();

    search_logic(bytes, query, limit)
}
