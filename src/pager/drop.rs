use super::Pager;

impl<const SIZE: usize> Drop for Pager<SIZE> {
    fn drop(&mut self) {
        let mut dirty_page_ids: Vec<usize> = vec![];
        for (&page_id, page) in self.cache.iter() {
            if page.borrow().is_dirty() {
                dirty_page_ids.push(page_id);
            }
        }

        for page_id in dirty_page_ids {
            if let Some(page) = self.cache.remove(page_id) {
                if let Err(e) = self.write_page(page_id, page.bytes()) {
                    eprintln!("Error when writing page {} on drop: {:?}", page_id, e);
                }
            }
        }
    }
}
