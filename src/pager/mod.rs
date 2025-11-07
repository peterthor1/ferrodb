use std::fs::File;
use std::fs::OpenOptions;
use std::io::Seek;
use std::io::SeekFrom;
use std::path::PathBuf;

mod lru;
pub mod page;

pub struct Pager<const SIZE: usize> {
    file_path: PathBuf,
    file_buffer: File,
    pages: usize,
    cache: lru::Lru<SIZE>,
}

impl<const SIZE: usize> Pager<SIZE> {
    pub fn new(
        file_path: &PathBuf,
        cache_size: usize,
    ) -> std::result::Result<Self, std::io::Error> {
        let mut file_buffer = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(file_path)?;

        let bytes = file_buffer.seek(SeekFrom::End(0))?;

        Ok(Self {
            file_path: file_path.to_path_buf(),
            file_buffer,
            pages: (bytes / SIZE as u64) as usize,
            cache: lru::Lru::new(cache_size),
        })
    }
}

mod drop;
mod interface;
mod io;
