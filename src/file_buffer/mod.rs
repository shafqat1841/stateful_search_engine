use std::{fs::File, path::PathBuf};

use memmap2::Mmap;

use crate::stateful_search_engine_errors::AllErros;

pub struct FileBuffer {
    data: Mmap,
}

impl FileBuffer {
    pub fn new(path: &PathBuf) -> Result<Self, AllErros> {
        let file: File = File::open(path)?;
        let mmap: Mmap = unsafe { Mmap::map(&file)? };

        Ok(FileBuffer { data: mmap })
    }

    pub fn get_bytes(&self) -> &[u8] {
        &self.data
    }
}
