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

fn show_result<'file_buffer>(
    query: &str,
    result: (String, Option<&Vec<SearchResult<'file_buffer>>>),
) {
    // match result.1 {
    //     None => {
    //         println!("No result found for this query: {:?}", query);
    //     }
    //     Some(val) => {
    //         if val.is_empty() {
    //             println!("No entries found for this query: {:?}", query);
    //         } else {
    //             for search_result in val {
    //                 let print_value: &SearchResult<'file_buffer> = search_result;
    //                 println!("{}", print_value)
    //             }
    //         }
    //     }
    // }

    let lru = result.0;
    println!("Least recently used value is: {}", lru);
}

fn search_logic<'file_buffer, 'cache>(
    bytes: &'file_buffer [u8],
    cache: &'cache mut Cache<'file_buffer>,
    query: String,
    limit: Option<usize>,
) -> Result<(), AllErros> {
    if cache.check_query(&query) {
        println!("from cache");
        let result: (String, Option<&Vec<SearchResult<'file_buffer>>>) =
            cache.get_result(&query);
        show_result(&query, result);
        return Ok(());
    }

    println!("from file");
    let mut log_searcher: LogSearcher<'file_buffer> = LogSearcher::new(bytes);

    let query_result: Vec<SearchResult<'file_buffer>> =
        log_searcher.search(&query, limit)?;

    cache.add_new_query_in_lrc_and_entries(query.clone(), query_result);

    let result: (String, Option<&Vec<SearchResult<'file_buffer>>>) =
        cache.get_result(&query);

    show_result(&query, result);

    Ok(())
}

pub fn run() -> Result<(), AllErros> {
    // let path: PathBuf = PathBuf::from("./log_files/test_access.log");
    let path: PathBuf = PathBuf::from("./log_files/access.log");
    let limit: Option<usize> = Some(1000);
    // let limit: Option<usize> = None;
    let mut cache: Cache = Cache::new();
    let file_buffer = FileBuffer::new(&path)?;

    let bytes: &[u8] = file_buffer.get_bytes();

    loop {
        let mut query_input = String::new();
        println!("Enter your query or type q for quiting the program");
        std::io::stdin().read_line(&mut query_input)?;

        let trimmed = query_input.trim();

        if trimmed == "q" {
            break;
        }

        search_logic(bytes, &mut cache, trimmed.to_string(), limit)?;
    }

    Ok(())
}
