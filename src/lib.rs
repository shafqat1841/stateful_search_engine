mod file_buffer;
mod stateful_search_engine_errors;

use std::path::PathBuf;

use crate::{file_buffer::FileBuffer, stateful_search_engine_errors::AllErros};

pub fn run() -> Result<(), AllErros> {
    let path: PathBuf = PathBuf::from("./log_files/access.log");
    let query: String = "302".to_string();
    let limit: usize = 1000;

    let file_buffer = FileBuffer::new(&path)?;

    let bytes: &[u8] = file_buffer.get_bytes();

    println!("{}",query);
    println!("{}",limit);
    println!("{:?}",bytes);

    Ok(())
}
