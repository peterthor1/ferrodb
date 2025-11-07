use std::cell::RefCell;

use super::Pager;
use crate::pager::page::Page;

impl<const SIZE: usize> Pager<SIZE> {
    pub fn get_page(
        &mut self,
        page_id: usize,
    ) -> std::io::Result<Option<&RefCell<Page<{ SIZE }>>>> {
        if self.cache.contains(page_id) {
            return Ok(Some(self.cache.get(page_id).unwrap()));
        }

        let page_data = self.read_page(page_id)?;
        if page_data.is_none() {
            return Ok(None);
        }
        let page = Page::new(page_id, &page_data.unwrap());

        if let Some(evicted_pages) = self.cache.add(page.id(), page).unwrap() {
            for page in evicted_pages {
                if page.is_dirty() {
                    self.write_page(page.id(), page.bytes())?;
                };
            }
        };

        Ok(Some(self.cache.get(page_id).unwrap()))
    }

    pub fn create_page(&mut self) -> std::io::Result<&RefCell<Page<{ SIZE }>>> {
        let page_id = self.pages;
        let zeroed_page = Box::new([0u8; SIZE]);

        self.write_page(self.pages, &zeroed_page)?;
        self.pages += 1;

        let page = Page::new(page_id, &zeroed_page);

        if let Some(evicted_pages) = self.cache.add(page.id(), page).unwrap() {
            for page in evicted_pages {
                if page.is_dirty() {
                    self.write_page(page.id(), page.bytes())?;
                };
            }
        };

        Ok(self.cache.get(page_id).unwrap())
    }
}
