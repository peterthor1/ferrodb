pub struct Page<const SIZE: usize> {
    id: usize,
    data: Box<[u8; SIZE]>,
    page_type: PageType,
    dirty: bool,
    pinned: bool,
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PageType {
    Free = 0x00,
    Meta = 0x01,
    IndexInternal = 0x02,
    IndexLeaf = 0x03,
    TableInternal = 0x04,
    TableLeaf = 0x05,
    Corrupt = 0x06,
}

impl<const SIZE: usize> Page<SIZE> {
    pub fn new(id: usize, data: &[u8; SIZE]) -> Self {
        Self {
            id,
            data: Box::new(*data),
            dirty: false,
            pinned: false,
            page_type: match data[0] {
                0x00 => PageType::Free,
                0x01 => PageType::Meta,
                0x02 => PageType::IndexInternal,
                0x03 => PageType::IndexLeaf,
                0x04 => PageType::TableInternal,
                0x05 => PageType::TableLeaf,
                _ => PageType::Corrupt,
            },
        }
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn pin(&mut self) {
        self.pinned = true;
    }

    pub fn unpin(&mut self) {
        self.pinned = false;
    }

    pub fn is_pinned(&self) -> bool {
        self.pinned
    }

    pub fn is_corrupt(&self) -> bool {
        self.page_type == PageType::Corrupt
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn page_type(&self) -> PageType {
        self.page_type
    }

    pub fn bytes(&self) -> &[u8; SIZE] {
        &self.data
    }

    pub fn bytes_mut(&mut self) -> &mut [u8; SIZE] {
        &mut self.data
    }
}
