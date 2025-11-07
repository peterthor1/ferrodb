use std::io::{Read, Seek, SeekFrom, Write};

use super::Pager;

impl<const SIZE: usize> Pager<SIZE> {
    pub(super) fn write_page(
        &mut self,
        page_id: usize,
        page_data: &[u8; SIZE],
    ) -> std::io::Result<()> {
        let _ = self
            .file_buffer
            .seek(SeekFrom::Start((SIZE * page_id).try_into().unwrap()))?;
        self.file_buffer.write_all(page_data)?;
        Ok(())
    }

    pub(super) fn read_page(&mut self, page_id: usize) -> std::io::Result<Option<Box<[u8; SIZE]>>> {
        let file_len = self.file_buffer.metadata()?.len();
        if (SIZE * page_id) as u64 >= file_len {
            return Ok(None);
        }
        let _ = self
            .file_buffer
            .seek(SeekFrom::Start((SIZE * page_id) as u64))?;
        let mut buffer = Box::new([0u8; SIZE]);
        self.file_buffer.read_exact(&mut *buffer)?;
        Ok(Some(buffer))
    }
}
