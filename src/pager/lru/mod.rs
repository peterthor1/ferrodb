use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

use crate::pager::page::Page;

pub struct Lru<const SIZE: usize> {
    capacity: usize,
    cache: HashMap<usize, RefCell<Page<SIZE>>>,
    order: RefCell<Vec<usize>>,
}

#[derive(Debug)]
pub enum LruError {
    PageAlreadyExists,
}

impl<const SIZE: usize> Lru<SIZE> {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity: if capacity == 0 { 1 } else { capacity },
            cache: HashMap::<usize, RefCell<Page<SIZE>>>::new(),
            order: RefCell::new(Vec::<usize>::new()),
        }
    }

    pub fn get(&self, page_id: usize) -> Option<&RefCell<Page<SIZE>>> {
        match self.cache.get(&page_id) {
            Some(page) => {
                let mut order = self.order.borrow_mut();
                if let Some(index) = order.iter().position(|&x| x == page_id) {
                    order.remove(index);
                    order.push(page_id);
                }
                Some(page)
            }
            None => None,
        }
    }

    pub fn add(
        &mut self,
        page_id: usize,
        page: Page<SIZE>,
    ) -> Result<Option<Vec<Page<SIZE>>>, LruError> {
        match self.cache.entry(page_id) {
            Entry::Occupied(_) => Err(LruError::PageAlreadyExists),
            Entry::Vacant(e) => {
                let mut order = self.order.borrow_mut();
                e.insert(RefCell::new(page));
                order.push(page_id);
                if self.cache.len() > self.capacity {
                    let mut i = 0;
                    let mut evicted_pages = vec![];

                    while i < order.len() && self.cache.len() > self.capacity {
                        let lru_page_id = order[i];

                        if let Some(page) = self.cache.get(&lru_page_id) {
                            if page.try_borrow().is_ok() && !page.borrow_mut().is_pinned() {
                                order.remove(i);
                                evicted_pages
                                    .push(self.cache.remove(&lru_page_id).unwrap().into_inner());
                            }
                        }
                        i += 1;
                    }
                    return Ok(Some(evicted_pages));
                }
                Ok(None)
            }
        }
    }

    pub fn remove(&mut self, page_id: usize) -> Option<Page<SIZE>> {
        let mut order = self.order.borrow_mut();
        if let Some(pos) = order.iter().position(|&x| x == page_id) {
            order.remove(pos);
        }
        self.cache.remove(&page_id).map(|page| page.into_inner())
    }

    pub fn contains(&self, page_id: usize) -> bool {
        self.cache.contains_key(&page_id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&usize, &RefCell<Page<SIZE>>)> {
        self.cache.iter()
    }
}
